import styled from "styled-components";
import Image from "next/image";
import * as S from "./style";

export const Navbar = () => {
    return (
        <S.Root>
            <Image src="/assets/logo.svg" width={48} height={48} alt="logo" />
            <S.SearchBarRoot>
                <Image src="/assets/search.svg" width={24} height={24} alt="search" />
                <S.SearchText >Search</S.SearchText>
            </S.SearchBarRoot>
            <S.GithubIconWrapper target="_blank" href="https://github.com/Solo-steven/toy-lang-in-rust">
                <Image src="/assets/github.svg" width={32} height={32} alt="github" />
            </S.GithubIconWrapper>
        </S.Root>
    )
}

export const NavbarSpacer = styled.div`
    min-height: ${props => props.theme.navbar.height};
`;
