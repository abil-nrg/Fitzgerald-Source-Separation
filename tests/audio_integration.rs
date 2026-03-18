use fitzgerald_source_separation::audio;


#[test]
fn test_audio_round_trip() {
    let input_path = "test_assets/test_tone.wav";
    let output_path = "test_assets/output_test.wav";

    let original = audio::load_audio(input_path).expect("Failed to load input");

    audio::save_wav(output_path, &original).expect("Failed to save output");

    let copy = audio::load_audio(output_path).expect("Failed to load copy");

    assert_eq!(original.sample_rate, copy.sample_rate);
    assert_eq!(original.channels, copy.channels);
    
    assert_eq!(original.samples.len(), copy.samples.len());
}