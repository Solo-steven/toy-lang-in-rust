import { css } from "styled-components";

const Theme = {
    typography : {
        titleLg: css`
            font-family: 'Montserrat';
            font-weight: 700;
            font-size: 48px;
            line-height: 72px;
            letter-spacing: 0.01em;
        `,
        titleSm: css`
            font-family: 'Montserrat';
            font-weight: 700;
            font-size: 38px;
            line-height: 57px;
        `,
        h1Lg: css`
            font-family: 'Montserrat';
            font-weight: 400;
            font-size: 38px;
            line-height: 57px;
        `,
        h1Sm: css`
            font-family: 'Montserrat';
            font-weight: 400;
            font-size: 28px;
            line-height: 42px;
        `,
        h2Lg: css`
            font-family: 'Montserrat';
            font-weight: 400;
            font-size: 28px;
            line-height: 42px;
        `,
        h2Sm: css`
            font-family: 'Montserrat';
            font-weight: 400;
            font-size: 20px;
            line-height: 30px;
        `,
        body1: css`
            font-family: 'Lato';
            font-weight: 400;
            font-size: 18px;
            line-height: 26px;
        `,
        body2: css`
            font-family: 'Lato';
            font-style: normal;
            font-weight: 400;
            font-size: 12px;
            line-height: 18px;
        `,
    },
    navbar: {
        height: 80
    },
    breakpoints: {
        xs: `@media (min-width: 0px)`,
        lg: `@media (min-width: 992px)`,
    }
}

export default Theme;