Goal: have some kind of interface like this

```rust
let mut bible_manager = BibleManager::default();
bible_manager.set("Genesis 1:1", "In the beginning, God created the heavens and the earth.");

// to get a specific verse
// parameter must impl ReferenceParse
_ = bible_manager.get_verse("Genesis 1:1"); // Option<String>
_ = bible_manager.get_verse("01001001");
_ = bible_manager.get_verse(1);


// iter/get_verses
_ = bible_manager.get_verses("Genesis 1:1"); // Option<Vec<String>>
_ = bible_manager.get_verses("Genesis 1:1-2");
```

Where I have 2 managers:
- BibleManager: just holds content for a verse
- MediaManager: holds related media that can span verses and do complex queries to determine overlapping segments

Goals:
- User should never have to parse the verse reference that they have before passing it, they should just be able to say `bible_manager.get("Genesis 1:1")`,
but a method like `bible_manager.is_valid("Genesis 1:1")` should be provided
- 
