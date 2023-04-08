import type { GetStaticProps, NextPage } from "next";
import { getMainPageDate } from "@/private/index";
import Image from "next/image";
import { SEOHeader } from "@/src/components/SEOHeader";
import { Navbar } from "@/src/components/Navbar";
import * as S from "./style";


type PageProps = ReturnType<typeof getMainPageDate>;

const Page: NextPage<PageProps> = ({
  title, introduction, articlePreviews,
}) => {
  return (
    <div>
      <SEOHeader title="Toy Lanugage In Rust" description="Build Your Own Lanuage In LLVM and Rust"/>
      <Navbar />
      <S.Background>
        <S.NavbarSpacer />
        <S.Container>
            <S.Title>{title}</S.Title>
            <S.ImageGroup>
              <S.ImageItemWrapper>
                <Image src="/assets/rust_logo.png" width={280} height={187.09} alt="rust-logo"/>
              </S.ImageItemWrapper>
              <S.ImageItemWrapper>
                <Image src="/assets/llvm_logo.svg" width={336} height={336} alt="llvm-logo"/>
              </S.ImageItemWrapper>
            </S.ImageGroup>
            <S.IntroductionTitle>{introduction.title}</S.IntroductionTitle>
            <S.IntroductionContent>{introduction.content}</S.IntroductionContent>
            {articlePreviews.map(articlePreview => (
              <S.ArticlePreviewRoot key={articlePreview.title}>
                <S.ArticlePreviewHeader>
                  <S.ArticlePreviewTitle>{articlePreview.title}</S.ArticlePreviewTitle>
                  <S.ArticlePreviewReadMore>Read More</S.ArticlePreviewReadMore>
                </S.ArticlePreviewHeader>
                <S.ArticlePreviewContent>{articlePreview.content}</S.ArticlePreviewContent>
              </S.ArticlePreviewRoot>
            ))
            }
        </S.Container>
      </S.Background>
    </div>
  )
}
export default Page;

export const getStaticProps: GetStaticProps<PageProps> = () => {
  return {
    props: getMainPageDate(),
  }
}