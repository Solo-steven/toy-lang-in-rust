import Head from "next/head";

interface SEOHeaderProps {
    title: string;
    description: string;
}

export const SEOHeader = ({ title, description }: SEOHeaderProps) => {
    return (
        <Head>
            <title>{title}</title>
            <meta name="description" content={description} />
        </Head>
    );
}

export default SEOHeader;