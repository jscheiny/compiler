## Tuples

### Named tuples / access
```
// Named
type Pair = (left: int, right: int);
let pair: Pair = (left = 5, right = 6); // Maybe?
let l: int = pair.left;
let r: int = pair.right;

// Unnamed
type Triple[T] = (T, T, T);
let triple: Triple[int] = (3, 4, 5);
let first = triple.1; // Zero index?
let second = triple.2;
let third = triple.3;
```

### Destructuring
```
let (a, b, c) = (1, 2, 3);
```

### Spreading

```
type Coordinate = (x: int, y: int);
fn magnitude(...coord: Coordinate): int -> coord.x + coord.y;

magnitude(3, 4);
let coordinate: Coordinate = (5, 6);
magnitude(...coordinate);
```

## Generics

### Generic functions

```
fn apply[T, U](value: T, applicator: T => U): U
    -> applicator(value);
```

### Generic structs / enums / interfaces

```
enum Option[T](
    Some(T),
    None
) {
    // Inner generic methods
    map[U](mapper: T => U): Option[U]
        -> match self {
            Some(let value) -> Some(mapper(value))
            None -> None
        };
}

struct Pair[T](first: T, second: T) { ... }

interface Plus[T, U] {
    plus(other: T): U;
}
```

### Bounds

```
fn sum[T: Plus[T, infer U]](left: T, right: T): U -> left + right;
```

### Variadics
```
fn apply[...Args](args: Args, applicator: (...Args) => T): T
    -> applicator(...args);

apply((3, 4, 5), (a, b, c) -> sqrt(a^2 + b^2 + c^2));
```

## Operator overloading

```
// Built in interfaces like:
interface Plus[T, U] {
    plus(other: T): U;
}

// Apply to your types:
struct Position(pub row: int, pub column: int) {
    impl Plus[Position, Position] {
        plus(other: Position): Position
            -> Position(@row + other.row, @column + other.column);
    }
}

// Use as a bound for generic types
```

## Nested type

```
struct Pair[T](left: T, right: T) {
    pub type Tuple = (left: T, right: T);
}

let x: Pair[int].Tuple = (3, 4);
```

```
interface Plus[T] {
    type Result;

    plus(other: T): Result;
}
```

## Misc

### Static methods

Option 1:
```
struct Coordinate(row: int, column: int) {
    static diagonal(value: T): Coordinate -> Coordinate(value, value);
    magnitude(): int -> @row + @column;
}

Coordinate.diagonal // [T] (T) => Coordinate[T]
```

Option 2:
```
struct Coordinate(row: int, column: int) {
    diagonal(value: T): Coordinate -> Coordinate(value, value);
    @magnitude(): int -> @row + @column; // use @ to indicate non-static
}
```
