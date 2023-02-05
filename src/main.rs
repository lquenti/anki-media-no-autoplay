const AUDIO_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/Bruh.mp3");
const VIDEO_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/aaaa.mp4");

use genanki_rs::*;

fn main() -> Result<(), Error> {
    /*
     * Note the Fields.
     * We have three fields: Question, Answer and Hidden.
     *
     * The front of the card will show whatever is in the Question field.
     * The back of the card will show whatever is in the Question field, followed by an <hr>,
     * followed by the Answer field.
     *
     * Note that the Hidden field is not used in the template.
     *
     */
    let my_model = Model::new(
        1607392319,
        "Simple Model",
        vec![
            Field::new("Question"),
            Field::new("Answer"),
            Field::new("Hidden"),
        ],
        vec![Template::new("Card 1")
            .qfmt("{{Question}}")
            .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
    );

    let audio_note = Note::new(
        my_model.clone(),
        vec![
            "Question Text",
            "Answer Text
            <audio controls>
                <source src=\"Bruh.mp3\" type=\"audio/mpeg\">
            </audio>",
            "[sound:Bruh.mp3]",
        ],
    )?;
    let video_note = Note::new(
        my_model.clone(),
        vec![
            "Question Text",
            "Answer Text
            <video width=\"320\" height=\"240\" controls>
                <source src=\"aaaa.mp4\" type=\"video/mp4\">
            </video>",
            "[sound:aaaa.mp4]",
        ],
    )?;

    // The rest is boilerplate
    let mut my_deck = Deck::new(2059400110, "Testdeck Media", "Example Description");
    my_deck.add_note(audio_note);
    my_deck.add_note(video_note);
    let mut my_package = Package::new(vec![my_deck], vec![AUDIO_PATH, VIDEO_PATH])?;
    my_package.write_to_file("output.apkg")?;
    Ok(())
}
