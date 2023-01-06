# Tales of Two Wastelands Multilanguage Audio Patcher
Tales of Two Wastelands Multilanguage Audio Patcher, to be able to play TTW with mostly non-english audio.

It's a little program I made in a few hours that will take your language audio and will move it to the new locations TTW uses for audio, making it possible to play most of the TTW mod with audio in your language instead of english.

Also, this only covers audio. For text you need to translate the game yourself with ESP-ESM translator.

# How to use
* Install Fallout: New Vegas and Fallout 3 in your language, then copy both of their data folders to another folder.
* Install Fallout: New Vegas and Fallout 3 in english.
* Install Tales of Two Wastelands following the official tutorials.
* Now go to ModOrganizer mods folder, and create two new folders: "TTW Voices (your language)" and "TTW Voices (English)".
* Use a bsa unpacker to extract the "sound/voices" folder from all the bsa files in your language's copies of New Vegas and 3 into "TTW Voices (your language)".
* Use a bsa unpacker to extract the "sound/voices" folder from all the bsa files in the "Tales of Two Wastelands" folder into "TTW Voices (English)".
* Open a cmd, and run this program: *ttw_audio_patcher.exe "path/to/TTW Voices (English)" "path/to/TTW Voices (your language)"*
* It'll take a few minutes, and when it's done, close the cmd, and in ModOrganizer enable the  "TTW Voices (your language)" mod and make sure it loads after Tales of Two Wastelands.
* Done. Your game should now have voices mostly in your language, specially in the DC Wasteland part of the mod.

# Important info

This is a little program I made in a few hours because I didn't found any alternatives, so it pretty much "guesses" what files should go where based on name, path and hash. There are probably better ways to do it, but this worked for me well enough.
