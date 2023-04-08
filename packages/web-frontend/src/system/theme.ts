import { css } from "styled-components";

const Theme = {
    typography : {
        title: css`
            font-family: 'Montserrat';
            font-weight: 700;
            font-size: 48px;
            line-height: 72px;
            letter-spacing: 0.01em;
        `,
        h1: css`
            font-family: 'Montserrat';
            font-style: normal;
            font-weight: 400;
            font-size: 38px;
            line-height: 57px;
        `,
        h2: css`
            font-family: 'Montserrat';
            font-weight: 400;
            font-size: 28px;
            line-height: 42px;
        `,
        body1: css`
            font-family: 'Lato';
            font-weight: 400;
            font-size: 16px;
            line-height: 24px;
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
    }
}

export default Theme;