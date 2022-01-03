
export const PROJECTS: Project[] = [
  {
    name: "temple",
    description: "A platformer built on the Bevy Game Engine.",
    urls: [{ title: "Repository", url: "https://github.com/ChristopherJMiller/temple"}],
    tags: ["Rust", "Game"]
  },
  {
    name: "gazer",
    description: "A K8s CRD to handle static site deployments.",
    urls: [{ title: "Repository", url: "https://github.com/realliance/gazer"}],
    tags: ["Rust", "k8s", "Docker"]
  },
  {
    name: "thirsty",
    description: "An IoT solution to monitoring plant moisture.",
    urls: [
      {
        title: "Frontend",
        url: "https://github.com/ChristopherJMiller/thirsty-ui"
      },
     {
        title: "MQTT-SQL Broker",
        url: "https://github.com/ChristopherJMiller/thirsty-broker"
     },
     {
       title: "Shared Library",
       url: "https://github.com/ChristopherJMiller/thirsty-support"
     }
    ],
    tags: ["Rust", "PosgresQL", "Embedded", "Esp232", "MQTT", "React", "Docker"]
  },
  {
    name: "Open Fightstick",
    description: "A Rust solution for using Arduino Uno's as a joystick",
    urls: [{ title: "Repoistory", url: "https://github.com/ChristopherJMiller/ofs" }],
    tags: ["Rust", "Embedded", "Arduino", "Typescript"]
  },
  {
    name: "ArduinoGuitarHero",
    description: "An Arduino library and instructions for using a Wii Guitar Hero Controller for PC play",
    urls: [{ title: "Repository", url: "https://github.com/ChristopherJMiller/ArduinoGuitarHero" }],
    tags: ["C", "Embedded", "Arduino"]
  },
  {
    name: "June",
    description: "A blog api endpoint generator for use with webpack single page applications.",
    urls: [{ title: "Repository", url: "https://github.com/ChristopherJMiller/june" }],
    tags: ["Typescript", "Webpack"]
  },
  {
    name: "Prolog Pokemon",
    description: "A tool to generate decent pokemon team type coverage, using Prolog.",
    urls: [{ title: "Repository", url: "https://github.com/ChristopherJMiller/prolog-pokemon" }],
    tags: ["Prolog"]
  },
  {
    name: "MHW Farmhand",
    description: "A webapp to aid in resource gathering for the game Monster Hunter World.",
    urls: [{ title: "Repository", url: "https://github.com/ChristopherJMiller/mhw-farmhand" }],
    tags: ["Typescript", "React"]
  },
  {
    name: "libmahjong",
    description: "An engine for running games of Riichi Mahjong",
    urls: [{ title: "Repository", url: "https://github.com/realliance/libmahjong" }],
    tags: ["C++", "Game"]
  },
  {
    name: "gametable",
    description: "A deployable instance of libmahjong to be used for remote match play.",
    urls: [{ title: "Repository", url: "https://github.com/realliance/gametable" }],
    tags: ["C++", "Game", "Docker"]
  },
  {
    name: "beatcopy",
    description: "A utility for installing Beatsaber songs easier via directory watching.",
    urls: [{ title: "Repository", url: "https://github.com/ChristopherJMiller/beatcopy" }],
    tags: ["Elixir"]
  },
  {
    name: "LetsMeet",
    description: "A Discord Bot to generate WhenToMeet events",
    urls: [{ title: "Repository", url: "https://github.com/ChristopherJMiller/letsmeet" }],
    tags: ["Ruby", "Docker"]
  },
  {
    name: "chrismiller.xyz",
    description: "This website's dev environment is built with Snowpack and pug for easy management of dependencies and fast production performance.",
    urls: [{ title: "Repository", url: "https://github.com/ChristopherJMiller/chrismiller.xyz" }],
    tags: ["Snowpack", "Typescript", "Pug", "Docker"]
  }
];

export interface ProjectURL {
  title: string,
  url: string,
}

export interface Project {
  name: string,
  description: string,
  urls: ProjectURL[],
  tags?: string[]
}
