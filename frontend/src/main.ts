import './style.css'
import { searchCallback } from './types.ts'
import { setupCounter } from './searchbar.ts'
import { projects } from './testdata.ts'
import { setupProject } from './project.ts'


function search() {
  const projectsDiv = document.querySelector<HTMLDivElement>("#results");

  projects.map((p) => {
    const projectsElement = document.createElement('a')
    projectsElement.id = 'project' + p.id;
    projectsElement.classList.add('project');
    projectsDiv?.appendChild(projectsElement);

    setupProject(projectsElement, p)
  })
}


setupCounter(document.querySelector<HTMLInputElement>('#searchBox')!, search)




