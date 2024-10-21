import { portfolio } from './generated/portfolio';


export const projects: Array<portfolio.Project> = Array.from({ length: 100 }, (_, index) => index + 1).map((i) => {
    const exampleProject: portfolio.Project = new portfolio.Project();
    exampleProject.id = i;
    exampleProject.title = "Project " + i;
    exampleProject.description = "A longer description of the project is found here."
    exampleProject.tags = ["a", "b", "c", "project" + i];
    Array.from({ length: 100 }, (_, index) => index + 1).map((i) => {
        exampleProject.tags.push(i.toString());
    })
    return exampleProject;
});


