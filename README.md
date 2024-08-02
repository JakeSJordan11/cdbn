This is a command line tool for me to use to bundle a copy of all of my musical ideas that I write into a single place that I can update at any time without missing files, having duplicate files or just outdated recordings.

This tool assumes that you use the same organizational structure that I do for storing my DAW sessions.

The folder structure is as follows:
In your ~/Music/Ableton (I should add others like Logic) directory there should be a subdirectory holding all of your sessions. I name mine "Project Library" this is where your current working directory should be when running this script since it is going to be recursively going through your tree to find your files.

You may have many nested folders of any structure, but at the project root you need to have a Renders directory. This is what this script will look for to find all .mp3 files within them. I normally also have .wav, but I don't think I want to have them in a bundle just for storge and these ideas don't need to be super high quality.

## Folder Structure Examples:
Generic

    ~/Music/Ableton/
    └── Library of Projects/
        ├── Category of Projects/
        │   ├── Poject Directory/
        │   │   ├── Project 1/
        │   │       └── ... project files
        │   │   ├── Project 1 (versioned)/
        │   │       └── ... project files
        │   │   ├── Project 2/
        │   │       └── ... project files
        │   │   ├── Project 2 (versioned)/
        │   │       └── ... project files
        │   │   └── Renders/
        │   │       ├── project 1.mp3
        │   │       ├── project 2.mp3
        │   │       └── ... other audio files


From my Project Library

    ~/Music/Ableton/
    └── Project Library/
        ├── Dated Ideas/
        │   ├── 1-3-2022/
        │   │   ├── 1-3-2022 Project/
        │   │       └── ... project files
        │   │   └── Renders/
        │   │       ├── 1-3-2022.mp3
        │   │       └── ... other audio files
        │   ├── 1-10-2021/
        │   │   ├── 1-10-2021 Project/
        │   │       └── ... project files
        │   │   └── Renders/
        │   │       ├── 1-10-2021.mp3
        │   │       └── ... other audio files
        │   └── ... other dated projects/