import { LruList } from '../Lru'

test('empty', () => {
  let lru = new LruList();
  expect(lru).not.toBeNull();
  expect(lru.count).toBe(0);
  expect (lru.pop(1)).toStrictEqual([]);
  expect (lru.pop(0)).toStrictEqual([]);
  expect(lru.count).toBe(0);
});

test('push and pop once', () => {
  let lru = new LruList<number>();
  let node1 = lru.push(1);
  const item1 = lru.access(node1);
  expect(item1).toBe(1);
  expect(lru.count).toBe(1);
  let popped = lru.pop(1);
  expect(lru.count).toBe(0);
  expect(popped).toStrictEqual([1]);
});

test('push and pop multiple', () => {
  let lru = new LruList<number>();
  lru.push(1);
  lru.push(2);
  lru.push(3);
  expect(lru.count).toBe(3);
  let popped = lru.pop(1);
  expect(lru.count).toBe(2);
  expect(popped).toStrictEqual([3]);
});

test('access', () => {

  let lru = new LruList<number>();

  const node1 = lru.push(1);
  const node2 = lru.push(2);
  const node3 = lru.push(3);

  expect(lru.count).toBe(3);

  const item2 = lru.access(node2);
  expect(item2).toBe(2);
  expect(lru.head).toBe(2);
  expect(lru.tail).toBe(3);

  const item3 = lru.access(node3);
  expect(item3).toBe(3);
  expect(lru.head).toBe(3);
  expect(lru.tail).toBe(1);

  const popped = lru.pop(1);

  expect(lru.count).toBe(2);
  expect(popped).toStrictEqual([1]);

  const popped2 = lru.pop(2);

  expect(lru.count).toBe(0);
  expect(popped2).toStrictEqual([2, 3]);
});