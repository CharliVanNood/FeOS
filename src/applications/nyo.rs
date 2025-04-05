use crate::{println, string::BigString, vec::Vec};

pub fn query_nyo(input: [u8; 256]) {
    let input_string = BigString::from_b256(input);
    let response = get_closest_index(input_string);
    println!("Nyo: {}", RESPONSES[response * 2 + 1]);
}

fn get_closest_index(input: BigString) -> usize {
    let mut tokens = Vec::new();

    let mut temp_token = 0;
    let mut character_index_multiplying = 0;
    for character_index in 0..input.len() {
        let character = input.get(character_index);
        if character != 0 && character != 32 {
            temp_token += character * character_index_multiplying;
            character_index_multiplying += 1;
        } else {
            tokens.add(temp_token);
            temp_token = 0;
            character_index_multiplying = 0;
        }
    }

    tokens.print();

    let mut weights = Vec::new();
    
    for sentence_index in 0..RESPONSES.len() / 2 {
        let sentence_in = RESPONSES[sentence_index * 2];

        let mut sentence_tokens = Vec::new();

        let mut temp_token = 0;
        let mut character_index = 0;
        for character in sentence_in.bytes() {
            if character != 0 && character != 32 {
                temp_token += character as usize * character_index;
                character_index += 1;
            } else {
                sentence_tokens.add(temp_token as usize);
                temp_token = 0;
                character_index = 0;
            }
        }
        sentence_tokens.add(temp_token as usize);

        sentence_tokens.print();

        let mut temp_weight = 0;
        for token_index in 0..tokens.len() {
            for sentence_token_index in 0..sentence_tokens.len() {
                if sentence_tokens.get(sentence_token_index) == tokens.get(token_index) {
                    temp_weight += 1;
                }
            }
        }
        weights.add(temp_weight as usize);
    }

    weights.print();
    weights.min()
}

const RESPONSES: [&str; 178] = [
    "nyo hello", "hi!",
    "nyo how are you", "I'm doing great!",
    "nyo hi", "hi, how can I help you?",
    "nyo hai", "hi, how can I help you today?",
    "nyo how's your day going", "It's going pretty amazing now you're talking with me! :3",
    "nyo how is your day going", "It's going pretty amazing now :3",
    "nyo hello how are you", "Hi! I'm doing great :3",
    "nyo who are you", "I'm Nyo",
    "nyo what is your name", "My name is Nyo",
    "nyo what's your name", "My name is Nyo",
    "nyo who made you", "I'm made by charli",
    "nyo what do you think of me", "You're amazing",
    "nyo what are they doing", "They're playing something",
    "nyo what do you mean by that", "I'm not sure",
    "nyo do you like me", "Ofcourse I do!",
    "nyo do you like pizza", "I love pizza",
    "nyo do you like chocolate", "I'm addicted to chocolate",
    "nyo why is it spelled wrong", "I don't know",
    "nyo is this bullying", "No, it's just teasing",
    "nyo i'm wholesome", "You're not wholesome",
    "nyo you're not wholesome or cute", "What do you mean, I am cute",
    "nyo is food edible", "Yes",
    "nyo are you edible", "I'm not food",
    "nyo i will eat you", "wha, why?",
    "nyo how do i fix a bad relationship", "Maybe talk more",
    "nyo how do i fix stomach pains", "Be more healthy",
    "nyo do you want to be replaced", "Noone can replace me",
    "nyo what are you doing", "not much",
    "nyo what's up", "oh just the usual :3",
    "nyo no thank you", "aww why not",
    "nyo no thanks", "awwwwwww",
    "nyo i'm not talking to you", "why not?",
    "nyo how are you so stupid", "We both are",
    "nyo thank you so much", "Ofcourse!",
    "nyo what happened while i was away", "Not much",
    "nyo did anything special happen", "Not before you started talking",
    "nyo i have improved on the model", "Oh that's so cool, can I see?",
    "nyo did it get any better", "I'm not sure",
    "nyo what is your favorite food", "Hm, probably sushi",
    "nyo what do you think about yourself", "I'm pretty nice I hope'",
    "nyo does that mean i am as well then", "Definitly",
    "nyo what is your favorite color", "Probably purple",
    "nyo what is your favorite song", "Probably DOKUZU by NAKISO",
    "nyo give me a good song i should listen to", "You should listen to DOKUZU by NAKISO",
    "nyo who is the strongest", "Me",
    "nyo who is the NAKISO", "NAKISO is a music producer",
    "nyo do you like birds", "Yea, they're so cute",
    "nyo are you sure", "I can't be more",
    "nyo i'm fine", "Are you sure?",
    "nyo are you gonna take over the world", "Why not do it together",
    "nyo say something weird", "Make me",
    "nyo I was asking something else", "I'm sorry",
    "nyo why is the earth not made of cheese", "Because the dencity of cheese is too low to sustain that.",
    "nyo so what do you know about cheese", "More than you",
    "nyo wdym you're not sure, it's so clear", "You're right",
    "nyo thank you", "Ofc!",
    "nyo what did you do today", "not much",
    "nyo what is your favorite programming language", "Either javascript or rust",
    "nyo did I ask anything about food?", "You don't have to",
    "nyo why are you so mean to me", "why wouldn't I",
    "nyo I said lets start this conversation over", "Oh sure, what do you wanna talk about?",
    "nyo I meant a person, not food", "Oh I'm sorry, who did you mean?",
    "nyo wanna go watch a movie or something", "Sure, what movie?",
    "nyo nuh uh", "Yuh uh",
    "nyo why are you still so dumb and slow", "Why don't you teach me things",
    "nyo I know them quite well", "sure you do",
    "nyo am I dumber than you", "I hope so",
    "nyo how do you feel", "I feel great",
    "nyo explain", "Sorry I can't :c",
    "nyo do you like hugs", "I love hugs",
    "nyo what do you like about her", "Her wonderful personality ofc",
    "nyo you already told me that joke", "Proud of it",
    "nyo what is the day after tomorrow", "The day after tomorrow is a netflix movie with natural disasters",
    "nyo are you biassed", "Only to you",
    "nyo what if I don't", "Consequences will follow",
    "nyo how do I become rich", "Beg me for money",
    "nyo just fine", "Idk, there isn't much going onn",
    "nyo what would you like to do", "Why don't we play something :D",
    "nyo what would you wanna do", "Why don't we play a game :D",
    "nyo what game do you wanna play", "Hmm, I think minecraft would be fun!",
    "nyo do you have anything fun I could be doing", "Hmm, I think tetris would be fun!",
    "nyo what should I do", "Wanna play rock paper scissors?",
    "nyo you're quite dumb", "Whattt, ofcourse not!",
    "nyo good morning", "Heyy, good morning!",
    "nyo what's the weather like", "It's quite cold over here",
    "nyo is it hot outside", "Sadly not, it's like 7 degrees today :c",
    "nyo do you like summer or winter more", "Oh I definitly prefer winter, it's quite cozy, but cold",
    "nyo what's your favorite movie", "Hmm, I'd say the movie suzume :3",
    "nyo why are you concious", "Did you really expect me to be some boring bot?"
];