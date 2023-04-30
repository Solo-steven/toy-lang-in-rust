import MainPageData from "./main";

interface MainPageItem {
    title: string;
    content: Array<string> | string;
}
interface GetMainPageData {
    title: string;
    introduction: MainPageItem;
    articlePreviews: Array<MainPageItem>;
}

export function getMainPageDate(): GetMainPageData {
    return MainPageData;
}