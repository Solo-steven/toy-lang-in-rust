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
    ${props => props.theme.breakpoints.xs} {
        padding: 0px 30px;
    }

`;

export const Title = styled.div`
    ${props => props.theme.typography.titleLg}
    color: #F2F0FF;
    margin: 78px 0px 15px 0px;
    ${props => props.theme.breakpoints.xs} {
        ${props => props.theme.typography.titleSm};
        text-align: left;
    }
    ${props => props.theme.breakpoints.lg} {
        ${props => props.theme.typography.titleLg}
        text-align: center;
    }
`;

export const ImageGroup = styled.div`
    display: flex;
    align-items: center;
    justify-content: center;
    ${props => props.theme.breakpoints.xs} {
        margin-bottom: 20px;
    }
    ${props => props.theme.breakpoints.lg} {
        margin: 0px;
    }
`;

export const RustImageWrapper = styled.div`
    position: relative;
    ${props => props.theme.breakpoints.xs} {
        width: 140px;
        height: 94px;
    }
    ${props => props.theme.breakpoints.lg} {
        width: 280px;
        height: 188px;
    }
`;
export const LLVMImageWrapper = styled.div`
    position: relative;
    ${props => props.theme.breakpoints.xs} {
        width: 140px;
        height: 140px;
        margin-left: 0px;
    }
    ${props => props.theme.breakpoints.lg} {
        width: 336px;
        height: 336px;
        margin-left: 85px;
    }
`;

export const IntroductionRoot = styled.div`
    margin-bottom: 40px;
    padding-bottom: 40px;
    border-bottom: 1px solid #B5B3BC;
`;
export const IntroductionTitle = styled.div`
    margin-bottom: 8px;
    color: #F2F0FF;
    ${props => props.theme.breakpoints.xs} {
        ${props => props.theme.typography.h1Sm};
        text-align: left;
    }
    ${props => props.theme.breakpoints.lg} {
        ${props => props.theme.typography.h1Lg};
        text-align: center;
    }
`;

export const IntroductionContent = styled.div`
    ${props => props.theme.typography.body1}
    color: #F2F0FF;
    text-align: left;
    padding-bottom: 40px;
    border-bottom: 1px solid #F2F0FF;
    margin-bottom: 25px;
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
    ${props => props.theme.typography.h2Lg};
    text-align: left;
    ${props => props.theme.breakpoints.xs} {
        ${props => props.theme.typography.h2Sm};
    }
    ${props => props.theme.breakpoints.lg} {
        ${props => props.theme.typography.h2Lg};
    }
    background: linear-gradient(135deg, #0368FF 0%, #FF3E95 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
`;

export const ArticlePreviewReadMore = styled.p`
    ${props => props.theme.typography.body1}
    color: #FFFFFF;
    position: relative;
    cursor: pointer;
    display: inline-block;
    overflow: hidden;
    ${props => props.theme.breakpoints.xs} {
        margin-left: 0px;
        margin-top: 12px;
    }
    ${props => props.theme.breakpoints.lg} {
        margin-left: 20px;
        margin-top: 0px;
    }
    &::after {
        content: ' ';
        min-height: 1px;
        background-color: #FFFFFF;
        width: 100%;
        position: absolute;
        bottom: 1px;
        left: -100%;
    }
    &:hover {
        &::after {
            left: 0px;
            transition: all .15s ease-in-out;
        }
    }
`;

export const ArticlePreviewContent = styled.p`
    ${props => props.theme.typography.body1};
    display: -webkit-box;
    -webkit-box-orient: vertical;
    line-clamp: 3;
    -webkit-line-clamp: 3;
    overflow: hidden;
    color: #FFFFFF;
    & + & {
        margin-top: 8px;
    }
`;

export const DisplayNoneWhenInLg = styled.div`
    ${props => props.theme.breakpoints.xs} {
        display: block;
    }
    ${props => props.theme.breakpoints.lg} {
        display: none;
    }
`;

export const DisplayNoneWhenInSm = styled.div`
    ${props => props.theme.breakpoints.xs} {
        display: none;
    }
    ${props => props.theme.breakpoints.lg} {
        display: block;
    }
`;