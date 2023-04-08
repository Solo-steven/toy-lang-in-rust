import styled from "styled-components";

export const Root = styled.div`
    display: flex;
    align-items: center;
    background-color: #222222;
    height: 80px;
    padding: 16px 40px;
    position: fixed;
    top:0px;
    width: 100vw;
`;

export const SearchBarRoot = styled.div`
    display: flex;
    align-items: center;
    border-radius: 10px;
    margin-left: 50px;
    background-color: #221C3E;
    flex: 1;
    padding: 5px 25px;
    cursor: pointer;
`;
export const SearchText = styled.div`
    font-weight: 400;
    font-size: 16px;
    line-height: 24px;
    color: #FFFFFF;
    margin-left: 10px;
`;

export const GithubIconWrapper = styled.a`
    text-decoration: none;
    margin-left: 100px;
    cursor: pointer;
`;

