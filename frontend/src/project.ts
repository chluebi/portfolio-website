import { setupTag } from './tag.js';
import { Project, SearchCallback } from './types.js';

export function setupProject(linkElement: HTMLAnchorElement, projectData: Project, callback: SearchCallback) {
    let unfolded = false;

    linkElement.addEventListener('click', () => {
        unfolded = !unfolded;
        reload();
    })
    linkElement.addEventListener('keydown', (event) => {
        if (event.key === 'Enter' || event.key === ' ') {
            event.preventDefault();
            unfolded = !unfolded;
            reload();
        }
    })

    function reload() {
        linkElement.innerHTML = "";
        
        const titleElement = document.createElement('h2');
        titleElement.innerHTML = projectData.title;
        linkElement.appendChild(titleElement);

        linkElement.classList.remove("project-focus");
        linkElement.classList.remove("project-unfocus");
        if (unfolded) {
            linkElement.classList.add("project-focus")

            const descriptionElement = document.createElement('p');
            descriptionElement.innerHTML = projectData.description;
            linkElement.appendChild(descriptionElement);
        } else {
            linkElement.classList.add("project-unfocus")
        }

        const tagsElement = document.createElement('div');
        tagsElement.classList.add('tags');

        projectData.tags.map((tag) => {
            const tagElement = document.createElement('a');
            tagElement.innerHTML = tag;
            tagElement.classList.add('tag');
            setupTag(tagElement, callback);

            tagsElement.appendChild(tagElement);
        })

        linkElement.appendChild(tagsElement);

    }
    reload();
}