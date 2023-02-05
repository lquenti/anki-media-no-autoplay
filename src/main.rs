/*
 * Path to the actual assets
 */
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

    /*
     * The second argument is a vector with the following elements:
     * 1. The text field for the "Question" Field as specified in the model
     * 2. The text field for the "Answer" Field
     * 3. The text field for the "Hidden" Field
     *
     * Thus, as specified in the model, the front will be
     * > Question Text
     * and the back will be
     * > Question Text
     * > <hr id="answer">
     * > Answer Text
     * > <audio controls>
     * >   <source src=\"Bruh.mp3\" type=\"audio/mpeg\">
     * > </audio>
     *
     * The audio controls create a browser-supported UI interface instead of Ankis auto-play-only
     * support provided by [sound:FILENAME.mp3].
     *
     * The auto play does not get triggered, as Hidden is neither embedded in the front nor the
     * back side. So why do we include it at all?
     *
     * Anki will ignore any media that is not referenced in at least one note. Thus, we just
     * reference it so that it will be available in the Anki media so that we can use the
     * browser-powered methods for accessing it.
     *
     */
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

    /*
     * Same idea but for videos.
     * Even more important so that we do not have an external video player popping up
     */
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

    // Boilerplate
    let mut my_deck = Deck::new(2059400110, "Testdeck Media", "Example Description");
    my_deck.add_note(audio_note);
    my_deck.add_note(video_note);

    // Note that we have to include the initial files so that they can be packaged in the apkg
    // (which is a zip btw)
    let mut my_package = Package::new(vec![my_deck], vec![AUDIO_PATH, VIDEO_PATH])?;
    my_package.write_to_file("output.apkg")?;
    Ok(())
}
