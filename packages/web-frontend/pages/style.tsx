import styled from "styled-components";

export const Background = styled.div`
    background-color: #000000;
    height: 100vh;
    width: 100vw;
    display: flex;
    flex-direction: column;
    overflow-y: scroll;
`;

export const NavbarSpacer = styled.div`
    min-height: 80px;
`;

export const Container = styled.div`
    max-width: 1240px;
    margin: 0px auto;
    flex: 1;
`;

export const Title = styled.div`
    ${props => props.theme.typography.title}
    color: #F2F0FF;
    margin: 78px 0px 15px 0px;
    text-align: center;
`;

export const ImageGroup = styled.div`
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 70px;
`;
export const ImageItemWrapper = styled.div`
    & + & {
        margin-left: 85px;
    }
`;

export const IntroductionTitle = styled.div`
    ${props => props.theme.typography.h1}
    margin-bottom: 8px;
    color: #F2F0FF;
    text-align: center;
`;

export const IntroductionContent = styled.div`
    ${props => props.theme.typography.body1}
    color: #F2F0FF;
    text-align: left;
    padding-bottom: 40px;
    border-bottom: 1px solid #F2F0FF;
    margin-bottom: 18px;
`;

export const ArticlePreviewRoot = styled.div`
    margin-bottom: 35px;
`;

export const ArticlePreviewHeader = styled.div`
    display: flex;
    align-items: baseline;
    margin-bottom: 15px;
`;

export const ArticlePreviewTitle = styled.div`
    ${props => props.theme.typography.h2}
    width: 481px;
    height: 42px;
    background: linear-gradient(135deg, #0368FF 0%, #FF3E95 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-fill-color: transparent;
`;

export const ArticlePreviewReadMore = styled.p`
    ${props => props.theme.typography.body1}
    color: #FFFFFF;
    margin-left: 20px;
`;

export const ArticlePreviewContent = styled.p`
    ${props => props.theme.typography.body1}
    color: #FFFFFF;
`;