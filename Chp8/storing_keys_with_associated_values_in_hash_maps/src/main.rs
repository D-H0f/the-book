fn main() {
    println!("\x1b[1;4;33mStoring Keys with Associated Values in Hash Maps\x1b[0m 
    
    The last of our common collections is the hash map. The type Hashmap<K, V> stores a mapping of 
    keys of type K to values of type V using a hashing function, which determines how it places these 
    keys and values into memory. Many programming languages support this kind of data structure, but 
    they often use a different name, such as hash, map, object, hash table, dictionary, or associative array,
    just to name a few.

    Hash maps are useful when you want to look up data not by using an index, as you can with vectors,
    but by using a key that can be of any type. For example, in a game, you could keep track of each 
    team's score in a hash map in which each key is a team's name and the values are each team's score.
    Given a team name, you can retrieve its score.

    We'll go over the basic API of hash maps in this section, but many more goodies are hiding in the 
    functions defined on HashMap<K, V> by the standard library. As always, check the standard library 
    documentation for more information.

\x1b[1;4;33mCreating a New Hash Map\x1b[0m 

    One way to create an empty hash map is to use new and to add elemnts with insert. In the next example,
    we're keeping track of the scores of two teams whose names are Blue and Yellow. The Blue team 
    starts with 10 point, and the Yellow team starts with 50.\x1b[97m 
        use std::collectoins::HashMap;

        let mut scores = HashMap::new();
        
        scores.insert(String::from(\"Blue\"), 10);
        scores.insert(String::from(\"Yellow\"), 50);\x1b[0m");
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("
    Note that we need to first use the HashMap from the collections portion of the standard library. Of 
    our three common collectoins, this one is the least often used, so it's not included in the features 
    brought into scope automatically in the prelude. Hash maps also have less support from the 
    standard library; there's no built-in macro to construct them, for example.

    Just like vectors, hash maps store their data on the heap. This HashMap has keys of type String and 
    values of type i32. Like vectors, hash maps are homogeneous: all of the keys must have the same 
    type, and all of the values must have the same type.

\x1b[1;4;33mAccessing Values in a Hash Map\x1b[0m

    We can get a value out of the hash map by providing its key to the get method, as shown here:\x1b[97m 
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from(\"Blue\"), 10);
        scores.insert(String::from(\"Yellow\") 50);

        let team_name = String::from(\"Blue\");
        let score = scores.get(&team_name).copied().unwrap_or(0);\x1b[0m");
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("   The output of this code would be:\n\x1b[97m{}\x1b[0m", score);
    println!("
    Here, score will have the value that's associated with the Blue team, and the result will be 10. The 
    get method returns an Option<&V>; if there's no value for that key in the hash map, get will 
    return None. This program handles the Option by calling copied to get an Option<i32> rather 
    than an Option<&i32>, then unwrap_or to set score to zero if scores doesn't have an entry for 
    the key.

    We can iterate over each key-value pair in a hash map in a similar manner as we do with vectors,
    using a fow loop:\x1b[97m 
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        
        scores.insert(String::from(\"Blue\"), 10);
        scores.insert(String::from(\"Yellow\"), 50);

        for (key, value) in &scores {{
            println!(\"{{key}}: {{value}}\");
        }}\x1b[0m

    This code will print each pair in an arbitrary order:");
    for (key, value) in &scores {
        println!("\x1b[97m{key}: {value}\x1b[0m");
    }
    println!("
\x1b[1;4;33mHash Maps and Ownership\x1b[0m 

    For types that implement the Copy trait, like i32, the values are copied into the hash map. for 
    owned values like String, the values will be moves and the hash map will be the owner of those 
    values, as demonstrated here:\x1b[97m 
        use std::collections::HashMap;

        let field_name = String::from(\"Favorite color\");
        let field_value = String::from(\"Blue\");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value are invalid at this point, you would get an error if you
        // tried to use them after using them with .insert()\x1b[0m 

    We aren't able to use the variables field_name and field_value after they've been moved into the 
    hash map with the call to insert.

    If we insert references to values into the hash map, the values won't be moved into the hash map.
    The values that the references point to must be valid for at least as long as the hash map is valid.
    We'll talk more about these issues in the \"Validating References with Lifetimes\" section in Chapter 10.

\x1b[1;4;33mUpdating a Hash Map\x1b[0m 

    Although the number of key and value pairs is growable, each unique key can only have one value 
    associated with it at a time (but not vice versa: for example, both the Blue team and the Yellow team 
    could have the value 10 stored in the scores hash map).

    When you want to change the data in a hash map, you have to decide how to handle the case when 
    a key already has a value assigned. You could replace the old value with the new value, completely 
    disregarding the old value. You could keep the old value and ignore the new value, only adding the 
    new value if the key doesn't already have a value. Or you could combine the old value and the new 
    value. Let's look at how to do each of these.

    \x1b[1;33mOverwriting a Value\x1b[0m 

    If we insert a key and a value into a hash map and then insert that same key with a different value,
    the value associated with that key will be replaced. Even though the code next shown calls insert 
    twice, the hash map will only contain one key-value pair because we're inserting the value for the 
    Blue team's key both times.\x1b[97m 
        use std::collections::Hashmap;

        let mut scores = HashMap::new();

        scores.insert(String::from(\"Blue\"), 10);
        scores.insert(String::from(\"Blue\"), 25);

        println!(\"{{scores:?}}\");\x1b[0m");
    scores.insert(String::from("Blue"), 25);

    println!("    this code will have the output:\x1b[97m 
{scores:?}\x1b[0m 
    The original value of 10 has been overwritten.");
    println!("
\x1b[1;4;33mAdding a Key and Value Only If a Key Isn't Present\x1b[0m 

    It's common to check whether a particular key already exists in the hash map with a value and then 
    to take the following actions: if the key does exist in the hash map, the existing value should remain 
    the way it is; if the key doesn't exist, insert it and a value for it.

    Hash maps have a special API for this called entry that takes the key you want to check as a 
    parameter. The return value of the entry method is an enum called Entry that represents a value 
    that might or might not exist. Let's say we want to check whether the key for the Yellow team has a 
    value associated with it. If it doesn't, we want to insert the value 50, and the same for the Blue 
    team. Using the entry API, the code looks liken this:\x1b[97m 
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from(\"Blue\"), 25);

        scores.entry(String::from(\"Yellow\")).or_insert(50);
        scores.entry(String::from(\"Blue\")).or_insert(50);

        println(\"{{scores:?}}\");\x1b[0m");
    scores.entry(String::from("Blue")).or_insert(50);
    println!("    The output for this would be:\x1b[97m 
{scores:?}\x1b[0m");
    println!("
    The or_insert method on Entry is defined to return a mutable reference to the value for the 
    corresponding Entry key if that key exists, and if not, is inserts the parameter as the new value for 
    this key and returns a mutable reference to the new value. This technique is much cleaner than 
    writing the logic ourselves and, in addition, plays more nicely with the borrow checker.

    Running the code previously shown will print {{\"Yellow\": 50, \"Blue\": 25}}. The first call to entry will 
    insert the key for the Yellow team with the value 50 because the Yellow team doesn't have a value 
    already. The second call to entry will not change the hash map because the Blue team already has 
    the value 25.

    \x1b[1;33mUpdating a Value Based on the Old Value\x1b[0m 

    Another common use case fow hash maps is to look up a key's value and then update it based on the 
    old value. For instance, the next example shows code that counts how many times each word appears in 
    some text. We use a hash map with the words as keys and increment the value to keep track of how 
    many times we've seen that word. If it's the first time we've seen a word, we'll first insert the value 0.\x1b[97m 
        use std::collections::HashMap;

        let text = \"hello world wonderful world\";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {{
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }}

        println!(\"{{map:?}}\");\x1b[0m");
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("    The output of this code would be:\x1b[97m 
{map:?}\x1b[0m");
    println!("
    You might see the same key-value pairs printed in a different order: recall from the \"Accessing Values 
    in a Hash Map\" section that iterating over a hash map happens in an arbitrary order.

    The split_whitespace method returns an iterator over subslices, seperated by whitespace, of the 
    value in text. The or_insert method returns a mutable reference (&mut V) to the value for the 
    specified key. Here, we store that mutable reference in the count variable, so in order to assign to 
    that value, we must first dereference count using the asterisk (*). The mutable reference goes out 
    of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing 
    rules.
    
\x1b[1;4;33mHashing Functions\x1b[0m 

    By default, HashMap uses a hashing function called SipHash that can provide resistance to denial-of-
    service (DoS) attacks involving hash tables. This is not the fastest hashing algorithm available, but 
    the trade-off for better security that comes with the drop in performance is worth it. If you profile 
    your code and find that the default hash function is too slow for your purposes, you can switch to 
    another function by specifying a different hasher. A hasher is a type that implements the 
    BuildHasher trait. We'll talk about traits and how to implement them in Chaper 10. You don't 
    necessarily have to implement your own hasher from scratch; crates.io has libraries shared by other 
    Rust users that provide hashers implementing many common hashing algorithms.

\x1b[1;4;33mSummary\x1b[0m 
    
    Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs 
    when you need to store, access, and modify data. Here are some exercises you should now be 
    equipped to solve:

        1. Given a list of integers, use a vector and return the median (when sorted, the value in the 
            middle position) and mode (the value that occurs most often; a hash map will be helpful here)
            of the list.
        2. Convert strings to pig latin. The first consonant of each word is moved to the end of the word 
            and ay is added, so first becomes irstfay. Words that start with a vowel have hay added to the 
            end instead (apple becomes applehay). Keep in mind the details about UTF-8 encoding.
        3. Using a hash map and vectors, create a text interface to allow a user to add employee names 
            to a department in a company; for example, \"Add Sally to Engineering\" or \"Add Amir to Sales\".
            Then let the user retrieve a list of all people in a department or all people in the company by 
            department, sorted alphabetically.

    The standard library API documentation describes methods that vectors, strings, and hash maps 
    have that will be helpful for these exercises.

    We're getting into more complex programs in which operations can fail, so it's a perfect time to 
    discuss error handling. We'll do that next.");
}
