import { portfolio } from './generated/portfolio';

export function setupProject(linkElement: HTMLAnchorElement, projectData: portfolio.Project) {
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

        linkElement.classList.remove("project-focus")
        if (unfolded) {
            linkElement.classList.add("project-focus")

            const descriptionElement = document.createElement('p');
            descriptionElement.innerHTML = projectData.description;
            linkElement.appendChild(descriptionElement);
        }
    }
    reload();
}