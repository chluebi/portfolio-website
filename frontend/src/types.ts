export type Project = {
    id: number,
    title: string,
    description: string,
    url?: string,
    languages: string[],
    tags: string[]
}

export type ProjectResponse = {
    type: "projects",
    data: Array<Project>
}

export type CompletionResponse = {
    type: "completion",
    data: string
}

export type SearchCallback = (arg: String) => void;