export type Project = {
    id: number,
    title: string,
    description: string,
    url?: string,
    languages: string[],
    tags: string[]
}

export type SearchCallback = (arg: String) => void;