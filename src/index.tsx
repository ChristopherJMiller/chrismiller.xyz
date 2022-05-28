
import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom";
import { PROJECTS, TAGS } from "./lib";
import ProjectCard from "./card";
import TagList from "./tags";

const App = () => {
  const [filter, setFilter] = useState<string | null>(null);

  const projects = filter ? PROJECTS.filter((project) => project.tags ? project.tags.includes(filter) : false) : PROJECTS;

  const projectCards = projects.map((repo) => <ProjectCard 
    key={repo.name} 
    project={repo}
  />);

  console.log(TAGS)

  const handleTagSelected = (tag: string) => {
    if (filter === tag) {
      setFilter(null)
    } else {
      setFilter(tag)
    }
  }

  return (
    <>
      <TagList tags={TAGS} activeTag={filter} onTagSelected={handleTagSelected} />
      <hr />
      <div className="grid xs:grid-cols-2 grid-flow-row-dense sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
        {projectCards}
      </div>
    </>
  );
}

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById("root")
);
