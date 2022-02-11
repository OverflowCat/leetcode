/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

void f(struct TreeNode* node){
    if (node == NULL) return;
    struct TreeNode* temp;
    temp = node->right;
    node->right = node->left;
    node->left = temp;
    f(node->left);
    f(node->right);
}

struct TreeNode* invertTree(struct TreeNode* root){
    f(root);
    return root;
}