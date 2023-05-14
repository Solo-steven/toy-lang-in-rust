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
              <S.RustImageWrapper>
                <Image src="/assets/rust_logo.png" fill alt="rust-logo"/>
              </S.RustImageWrapper>
              <S.LLVMImageWrapper>
                <Image src="/assets/llvm_logo.svg" fill alt="llvm-logo"/>
              </S.LLVMImageWrapper>
            </S.ImageGroup>
            <S.IntroductionRoot>
              <S.IntroductionTitle>{introduction.title}</S.IntroductionTitle>
              {Array.isArray(introduction.content) ? 
                  introduction.content.map(singleLine => (
                    <S.IntroductionContent key={singleLine} >{singleLine}</S.IntroductionContent>
                  )) : <S.IntroductionContent>{introduction.content}</S.IntroductionContent>}
            </S.IntroductionRoot>
            {articlePreviews.map(articlePreview => (
              <S.ArticlePreviewRoot key={articlePreview.title}>
                <S.ArticlePreviewHeader>
                  <S.ArticlePreviewTitle>{articlePreview.title}</S.ArticlePreviewTitle>
                  <S.DisplayNoneWhenInSm>
                    <S.ArticlePreviewReadMore>Read More</S.ArticlePreviewReadMore>
                  </S.DisplayNoneWhenInSm>
                </S.ArticlePreviewHeader>
                {Array.isArray(articlePreview.content) ? 
                articlePreview.content.map(singleLine => (
                  <S.ArticlePreviewContent key={singleLine} >{singleLine}</S.ArticlePreviewContent>
                )) : <S.ArticlePreviewContent>{articlePreview.content}</S.ArticlePreviewContent>}
                <S.DisplayNoneWhenInLg>
                  <S.ArticlePreviewReadMore>Read More</S.ArticlePreviewReadMore>
                </S.DisplayNoneWhenInLg>
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