import { Project } from './types.js';


export const projects: Array<Project> = Array.from({ length: 100 }, (_, index) => index + 1).map((i) => {
    const exampleProject: Project = {
        id: i,
        title: "Project " + i,
        description: "A longer description of the project is found here.",
        languages: [],
        tags: Array.from({ length: 100 }, (_, i) => (i + 1).toString())
    }
    return exampleProject;
});


