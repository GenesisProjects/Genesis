//! Patricia Tree
//!
//! Genesis use this crate to update hash root of account states and transaction set.
//! [Patricia Tree Reference](https://github.com/ethereum/wiki/wiki/Patricia-Tree)
//!
//! ```
//!
//! ```

/// Patricia Tree Node
///
/// ## Usages
/// Implememnt data struct of **Patricia Tree Node**
/// * Encode node path to nibbles
/// * Decode nibbles to node path
/// ### Examples
/// ```rust,no_run
/// extern crate common;
/// extern crate gen_core;
/// fn main() {
///     #[macro_use]
///     use common::hash::*;
///     use gen_core::mpt::node::*;
///
///     // Instantialize leaf node
///     let leaf_node: TrieNode<String> = TrieNode::new_leaf_node(&vec![1,2,3], &"test".to_string());
/// }
/// ```
pub mod node;

/// Patricia Tree
///
/// ## Usages
/// Implememnt data struct of **Patricia Tree**
/// * Query value in the tree by the key
/// * Update value in the tree by the key
/// * Delete value in the tree by the key
/// * Sync data changes to db.
/// ### Examples
/// ```rust,no_run
/// extern crate common;
/// extern crate gen_core;
/// extern crate db;
///
/// fn main() {
///     use gen_core::mpt::trie::*;
///     use gen_core::blockchain;
///     use gen_core::transaction::Transaction;
///     use db::manager::DBManager;
///
///     // Initialize a **Patricia Tree** with a hash root and the shared db manager
///     let root = blockchain::last_block_tx_root().unwrap();
///     let mut manager = DBManager::default();
///     let test_db = manager.get_db("test");
///     let mut tree = Trie::<Transaction>::load(root, &test_db);
/// }
/// ```
pub mod trie;