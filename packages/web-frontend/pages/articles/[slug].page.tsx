import type { GetStaticPaths, GetStaticProps, NextPage } from "next";

const Page: NextPage = () => {
    return (
        <div></div>
    )
}
export default Page;

export const getStaticProps: GetStaticProps = ({ params }) => {
    return {
        props: {}
    }
};

export const getStaticPaths: GetStaticPaths = () => {
    return {
        paths: [],
        fallback: "blocking",
    }
}





