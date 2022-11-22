#include "stdbool.h"
#include "stddef.h"

// Definition for singly-linked list.
struct ListNode {
  int val;
  struct ListNode *next;
};

bool hasCycle(struct ListNode *head) {
  struct ListNode *walker = head;
  struct ListNode *runner = head;

  while (true) {
    if (runner == NULL)
      return false;
    walker = walker->next;
    runner = runner->next;
    if (runner == NULL)
      return false;
    runner = runner->next;
    if (runner == walker)
      return true;
  }
}


int main() {
  return 0;  
}