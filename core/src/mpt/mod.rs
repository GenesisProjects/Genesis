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
/// * Implememnt data struct of **Patricia Tree Node**
/// * Encode node path to nibbles
/// * Decode nibbles to node path
/// ### Examples
/// ```
/// use mpt::node::*;
///
/// // Instantialize leaf node
/// let leaf_node: TrieNode<String> = TrieNode::new_leaf_node(&encoded_path, &"test".to_string());
///
/// // Instantialize branch node
/// let mut new_branches: [TrieKey; MAX_BRANCHE_NUM] = [zero_hash!(); MAX_BRANCHE_NUM];
/// let branch_node: TrieNode<String> = TrieNode::new_branch_node(&new_branches, None);
///
/// // Instantialize extension node
/// let path = vec![1, 2, 3];
/// let encoded_path = encode_path(&path, false);
/// let ext_node: TrieNode<String> = TrieNode::new_extension_node(&encoded_path, &zero_hash!());
/// ```
pub mod node;

/// Patricia Tree
///
/// ## Usages
/// * Implememnt data struct of **Patricia Tree**
/// * Query value in the tree by the key
/// * Update value in the tree by the key
/// * Delete value in the tree by the key
/// * Sync data changes to db.
/// ### Examples
/// ```
/// use gen_core::mpt::trie::*;
/// use gen_core::blockchain;
/// use gen_core::transaction::Transaction;
///
/// // DB manager singleton
/// use db::manager::SHARED_MANAGER;
///
/// // Initialize a **Patricia Tree** with a hash root and the shared db manager
/// let root = blockchain::last_block_tx_root();
/// let mut tree = Trie::<Transaction>::load(root, &SHARED_MANAGER);
///
/// // Insert a new transaction into db
/// let tx = Transaction::mock();
/// let (key, encoded_rlp) = tx.encrype_sha256();
///
/// tree.update(&key, &tx);
///
/// println!("DB updated, the new trie root is {:?}", tree.root());
///
/// // Query a transaction
/// let target_tx = tree.get(&key);
///
/// ```
pub mod trie;