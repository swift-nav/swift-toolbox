export class LruList<T> {

    _head?: LruNode<T>
    _tail?: LruNode<T>

    _count: number

    private _value : string;

    public get count() : number {
        return this._count;
    }
 
    public get head() : T {
        return this._head.item;
    }
  
    public get tail() : T {
        return this._tail.item;
    }
 
    public constructor() {
        this._head = null;
        this._tail = null;
        this._count = 0;
    }

    public push(item: T): LruNode<T> {

		const lruNode = new LruNode(item, this._tail, null);

        if (this._tail === null) {
            console.assert(this._head === null, "head and tail must be null together");
            this._tail = lruNode;
            this._head = lruNode;
        } else {
            this._tail.next = lruNode;
            this._tail = lruNode;
        }

        this._count += 1;

        return lruNode;
    }

    public access(node: LruNode<T>): T {

        if (Object.is(this._head, this._tail)) {
            return this._head.item;
        }

        let prev = node.prev;
        let next = node.next;

        if (Object.is(this._tail, node)) {
            this.popTail();
        }

        if (prev !== null) {
            prev.next = next;
        }

        if (next !== null) {
            next.prev = prev;
        }

        node.prev = null;
        node.next = null;

        this._head.prev = node;
        node.next = this._head;

        this._head = node;

        return node.item;
    }

    popTail() : T {

        console.assert(this._head !== null && this._tail !== null, "head and tail must both be null");

        if (Object.is(this._head, this._tail)) {

            let tailItem = this._head.item;
            this._head = null;
            this._tail = null;

            return tailItem;

        } else {

            console.assert(this._head !== null && this._tail !== null,
                "head and tail must both be not null");

            const newTail = this._tail.prev;
            let tailItem = this._tail.item;
            this._tail.next = null;
            this._tail.prev = null;
            newTail.next = null;
            this._tail = newTail;

            return tailItem;
        }
    }

    public pop(count: number): T[] {

        const popped: T[] = [];
        for (let i = 0; i < count && this._count > 0; i++) {
            popped.push(this.popTail());
            this._count -= 1;
        }
        return popped;
    }
}

export class LruNode<T> {

    item: T 

    next?: LruNode<T>
    prev?: LruNode<T>

    constructor(item: T, prev?: LruNode<T>, next?: LruNode<T>) {
        this.item = item
        this.prev = prev
        this.next = next
    }
}