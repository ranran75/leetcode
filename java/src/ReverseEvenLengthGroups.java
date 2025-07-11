public class ReverseEvenLengthGroups {

    public static void main(String[] args) {
        //[1,1,0,6,5]
        ListNode head =
            new ListNode(1, new ListNode(1, new ListNode(0, new ListNode(6, new ListNode(5)))));
        ReverseEvenLengthGroups solution = new ReverseEvenLengthGroups();
        ListNode result = solution.reverseEvenLengthGroups(head);
        while (result != null) {
            System.out.print(result.val + " ");
            result = result.next;
        }
    }

    public static class ListNode {
        int val;
        ListNode next;

        ListNode() {
        }

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }

    public ListNode reverseEvenLengthGroups(ListNode head) {
        ListNode lastTail = null;
        ListNode thisHead = head;
        ListNode thisTail;
        int groupIndex = 1;
        while (thisHead != null) {
            ListNode temp = thisHead;
            int length = 1;
            while (temp.next != null && length < groupIndex) {
                length++;
                temp = temp.next;
            }
            thisTail = temp;
            if (length % 2 == 0) {
                ListNode prev = thisTail.next;
                ListNode cur = thisHead;
                int count = 0;
                while (cur != null && count++ < length) {
                    ListNode next = cur.next;
                    cur.next = prev;
                    prev = cur;
                    cur = next;
                }
                lastTail.next = thisTail;
                lastTail = thisHead;
                thisHead = thisHead.next;
            } else {
                lastTail = thisTail;
                thisHead = thisTail.next;
            }
            groupIndex++;
        }
        return head;
    }
}