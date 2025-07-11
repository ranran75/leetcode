import java.util.HashMap;
import java.util.Map;

//https://leetcode.cn/problems/lru-cache/
public class LRUCache {
    int capacity;
    LinkedList<Integer, Integer> linkedList;
    // key -> ListNode reference
    Map<Integer, ListNode<Integer, Integer>> map;

    public LRUCache(int capacity) {
        this.linkedList = new LinkedList<>();
        this.capacity = capacity;
        this.map = new HashMap<>(capacity);
    }

    // 1. update LRU 2. return value
    public int get(int key) {
        ListNode<Integer, Integer> node = map.get(key);
        if (node != null) {
            linkedList.remove(node);
            linkedList.addTail(node);
            return node.value;
        }
        return -1;
    }

    public void put(int key, int value) {
        ListNode<Integer, Integer> node = map.get(key);
        if (node != null) {
            linkedList.remove(node);
            linkedList.addTail(node);
            node.value = value;
            return;
        }
        if (map.size() >= capacity) {
            ListNode<Integer, Integer> removed = linkedList.removeHead();
            map.remove(removed.key);
        }
        ListNode<Integer, Integer> newNode = new ListNode<>(key, value);
        linkedList.addTail(newNode);
        map.put(key, newNode);
    }

    static class LinkedList<K, V> {
        ListNode<K, V> head;
        ListNode<K, V> tail;

        public LinkedList() {
            this.head = new ListNode<>(null, null);
            this.tail = new ListNode<>(null, null);
            head.next = tail;
            tail.prev = head;
        }

        private void addTail(ListNode<K, V> node) {
            ListNode<K, V> prev = tail.prev;
            prev.next = node;
            node.prev = prev;
            node.next = tail;
            tail.prev = node;
        }

        private void remove(ListNode<K, V> node) {
            ListNode<K, V> prev = node.prev;
            ListNode<K, V> next = node.next;
            prev.next = next;
            next.prev = prev;
        }

        private ListNode<K, V> removeHead() {
            ListNode<K, V> removing = head.next;
            ListNode<K, V> next = removing.next;
            head.next = next;
            next.prev = head;
            return removing;
        }

    }


    static class ListNode<K, V> {
        K key;
        V value;
        ListNode<K, V> prev;
        ListNode<K, V> next;

        ListNode(K key, V value) {
            this.key = key;
            this.value = value;
        }
    }
}
