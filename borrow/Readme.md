    Borrowing:
        Borrowing allows you to temporarily access the data of a value without taking ownership.
        The owner retains control of the value while it's being borrowed.
        Borrowing ensures that there's always exactly one owner of a value at any given time.

    Immutable Borrowing:
        Immutable borrowing, indicated by the & symbol, allows read-only access to the borrowed value.
        Multiple immutable borrows of a value can exist simultaneously.
        Immutable borrows ensure that the value cannot be modified during the borrow.

    Mutable Borrowing:
        Mutable borrowing, indicated by the &mut symbol, allows read-write access to the borrowed value.
        Only one mutable borrow of a value can exist at a time.
        Mutable borrows ensure exclusive access to the value and prevent data races by disallowing concurrent modification.

    Ensuring Ownership Rules:
        Borrowing allows Rust to maintain ownership rules without violating them.
        Owners can choose to temporarily lend their data to other parts of the program for specific operations without giving up ownership.

    Preventing Dangling References:
        Borrowing helps prevent the problem of dangling references by ensuring that borrowed values cannot outlive their owners.
        The borrow checker in Rust analyzes the code at compile time to ensure that references are always valid.