import './style.css'
import { setupSearch } from './searchbar.ts'
import { projects } from './testdata.ts'
import { setupProject } from './project.ts'


function search(s: String) {
  const projectsDiv = document.querySelector<HTMLDivElement>("#results");
  if (projectsDiv) {
    projectsDiv.innerHTML = "";
  }

  projects.map((p, i) => {
    const projectsElement = document.createElement('a')
    projectsElement.id = 'project' + p.id;
    projectsElement.classList.add('project');
    projectsElement.setAttribute("tabindex", "" + (i+1));
    projectsDiv?.appendChild(projectsElement);

    setupProject(projectsElement, p)
  })

  history.replaceState({}, 'Search Results', '/' + s);
}


setupSearch(document.querySelector<HTMLInputElement>('#searchBox')!, search)




