import React from 'react';
import { Project } from "./lib";

interface CardProps {
  project: Project
}

export default ({ project }: CardProps) => {
  const links = project.urls.map((url) => 
    <a className="bg-blue-500 dark:bg-blue-800 dark:hover:bg-blue-700 hover:bg-blue-400 w-full p-1 rounded text-white text-center" href={url.url}>{url.title}</a>
  );

  const tags = project.tags ? project.tags.map((tag) => <div className="dark:bg-blue-600 bg-cyan-700 text-white rounded m-1 p-1">{tag}</div>) : null;

  return (
    <div className={"dark:bg-stone-700 bg-white w-full h-96 shadow-md rounded-md px-2 py-4 transition-all ease-in-out delay-50 duration-200 hover:-translate-y-1 hover:scale-105 flex flex-col"}>
      <p className="text-xl">{project.name}</p>
      <hr />
      <p className="text-base my-2">{project.description}</p>
      <div className="flex flex-row flex-wrap">
        {tags}
      </div>
      <div className="grid gap-2 flex-1 content-end">
        {links}
      </div>
    </div>
  )
}
