# Todo CLI application

## TODOS
- [x] Add Todo Item
- [x] List all Todo Items
- [x] Remove Todo Item
- [x] Check Todo Item
- [ ] Uncheck Todo Item
- [ ] Perform commands Add, Remove or Check with multiple arguments
- [ ] Perform all actions by ID as well. (Every item should have a unique `id`)


### Commands

#### add
```bash
todo add "Practice rust daily" "Link WASM to Frontend" "Watch anime"
```
Create new to-do items. All todo items are written to a `todo.db` file in same directory.
From the example above I'm adding 3 to-do items.

#### list
```bash
todo list
```
List all to-do items. All todo items are read from the `todo.db` file.

#### check
```bash
todo check "rust daily"
```
Mark todo items as completed when provided a substring of the intented to-do item


#### remove
```bash
todo remove "rust daily"
```
Remove todo items from the list
