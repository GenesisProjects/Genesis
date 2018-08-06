use parity_wasm::{builder, elements};

pub fn update_call_index(instructions: &mut elements::Instructions, inserted_index: u32) {
    use parity_wasm::elements::Instruction::*;
    for instruction in instructions.elements_mut().iter_mut() {
        if let &mut Call(ref mut call_index) = instruction {
            if *call_index >= inserted_index { *call_index += 1}
        }
    }
}

#[derive(Debug)]
struct BlockEntry {
    /// Index of the first instruction (aka `Opcode`) in the block.
    start_pos: usize,
    /// Sum of costs of all instructions until end of the block.
    cpu_cost: u32,
}


struct Meter {
    /// All blocks in the order of theirs start position.
    blocks: Vec<BlockEntry>,

    // Stack of blocks. Each element is an index to a `self.blocks` vector.
    stack: Vec<usize>,
}

impl Meter {
    fn new() -> Meter {
        Meter {
            stack: Vec::new(),
            blocks: Vec::new(),
        }
    }

    /// Begin a new block.
    fn begin(&mut self, cursor: usize) {
        let block_idx = self.blocks.len();
        self.blocks.push(BlockEntry {
            start_pos: cursor,
            cpu_cost: 1,
        });
        self.stack.push(block_idx);
    }

    /// Finalize the current block.
    ///
    /// Finalized blocks have final cost which will not change later.
    fn finalize(&mut self) -> Result<(), ()> {
        self.stack.pop().ok_or_else(|| ())?;
        Ok(())
    }

    /// Increment the cost of the current block by the specified value.
    fn increment(&mut self, val: u32) -> Result<(), ()> {
        let stack_top = self.stack.last_mut().ok_or_else(|| ())?;
        let top_block = self.blocks.get_mut(*stack_top).ok_or_else(|| ())?;

        top_block.cpu_cost = top_block.cpu_cost.checked_add(val).ok_or_else(|| ())?;

        Ok(())
    }
}

fn inject_mem_stat(instructions: &mut elements::Instructions, mem_stat_func: u32) -> Result<(), ()> {
    use parity_wasm::elements::Instruction::*;
    let mut counter = 0;
    for instruction in instructions.elements_mut() {
        if let GrowMemory(_) = *instruction {
            *instruction = Call(mem_stat_func);
            counter += 1;
        }
    }
    Ok(())
}

fn add_mem_stat(module: elements::Module, ext_mem_stat_func: u32) -> elements::Module {
    use parity_wasm::elements::Instruction::*;

    let mut b = builder::from_module(module);
    b.push_function(
        builder::function()
            .signature().params().i32().build().with_return_type(Some(elements::ValueType::I32)).build()
            .body()
            .with_instructions(elements::Instructions::new(vec![
                GetLocal(0),
                GetLocal(0),
                I32Const(1 as i32),
                I32Mul,
                // todo: there should be strong guarantee that it does not return anything on stack?
                Call(ext_mem_stat_func),
                GrowMemory(0),
                End,
            ]))
            .build().build()
    );

    b.build()
}

pub fn inject_cpu_stat(
    instructions: &mut elements::Instructions,
    cpu_stat_func: u32,
) -> Result<(), ()> {
    use parity_wasm::elements::Instruction::*;

    let mut meter = Meter::new();

    // Begin an implicit function (i.e. `func...end`) block.
    meter.begin(0);

    for cursor in 0..instructions.elements().len() {
        let instruction = &instructions.elements()[cursor];
        match *instruction {
            Block(_) | If(_) | Loop(_) => {
                // Increment previous block with the cost of the current opcode.
                let instruction_cost = 1;
                meter.increment(instruction_cost)?;

                // Begin new block. The cost of the following opcodes until `End` or `Else` will
                // be included into this block.
                meter.begin(cursor + 1);
            }
            End => {
                // Just finalize current block.
                meter.finalize()?;
            },
            Else => {
                // `Else` opcode is being encountered. So the case we are looking at:
                //
                // if
                //   ...
                // else <-- cursor
                //   ...
                // end
                //
                // Finalize the current block ('then' part of the if statement),
                // and begin another one for the 'else' part.
                meter.finalize()?;
                meter.begin(cursor + 1);
            }
            _ => {
                // An ordinal non control flow instruction. Just increment the cost of the current block.
                let instruction_cost =1;
                meter.increment(instruction_cost)?;
            }
        }
    }

    // Then insert metering calls.
    let mut cumulative_offset = 0;
    for block in meter.blocks {
        let effective_pos = block.start_pos + cumulative_offset;

        instructions.elements_mut().insert(effective_pos, I32Const(block.cpu_cost as i32));
        instructions.elements_mut().insert(effective_pos+1, Call(cpu_stat_func));

        // Take into account these two inserted instructions.
        cumulative_offset += 2;
    }

    Ok(())
}


pub fn inject_resource_stat(module: elements::Module)
                          -> Result<elements::Module, elements::Module>
{
    // Injecting resource statistic external
    let mut mbuilder = builder::from_module(module);
    let import_sig = mbuilder.push_signature(
        builder::signature()
            .param().i32()
            .build_sig()
    );

    // mem stat
    mbuilder.push_import(
        builder::import()
            .module("env")
            .field("mem_stat")
            .external().func(import_sig)
            .build()
    );

    // cpu stat
    mbuilder.push_import(
        builder::import()
            .module("env")
            .field("cpu_stat")
            .external().func(import_sig)
            .build()
    );

    // back to plain module
    let mut module = mbuilder.build();

    // calculate actual function index of the imported definition
    //    (substract all imports that are NOT functions)

    let cpu_stat_func = module.import_count(elements::ImportCountType::Function) as u32 - 1;
    let mem_stat_func = module.import_count(elements::ImportCountType::Function) as u32 - 2;

    let mut error = false;

    // Updating calling addresses (all calls to function index >= `gas_func` should be incremented)
    for section in module.sections_mut() {
        match section {
            &mut elements::Section::Code(ref mut code_section) => {
                for ref mut func_body in code_section.bodies_mut() {
                    update_call_index(func_body.code_mut(), cpu_stat_func);
                    if let Err(_) = inject_cpu_stat(func_body.code_mut(), cpu_stat_func) {
                        error = true;
                        break;
                    }
                    update_call_index(func_body.code_mut(), mem_stat_func);
                    if let Err(_) = inject_mem_stat(func_body.code_mut(), mem_stat_func) {
                        error = true;
                        break;
                    }
                }
            },
            &mut elements::Section::Export(ref mut export_section) => {
                for ref mut export in export_section.entries_mut() {
                    if let &mut elements::Internal::Function(ref mut func_index) = export.internal_mut() {
                        if *func_index >= mem_stat_func { *func_index += 2}
                    }
                }
            },
            &mut elements::Section::Element(ref mut elements_section) => {
                for ref mut segment in elements_section.entries_mut() {
                    // update all indirect call addresses initial values
                    for func_index in segment.members_mut() {
                        if *func_index >= mem_stat_func { *func_index += 2}
                    }
                }
            },
            _ => { }
        }
    }

    if error { return Err(module); }

    Ok(add_mem_stat(module, mem_stat_func))
}
