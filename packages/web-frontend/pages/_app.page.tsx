import type { AppProps } from 'next/app';
import { Navbar, NavbarSpacer } from "@/src/components/Navbar";
import { ThemeProvider } from "styled-components";
import theme from "@/src/system/theme";
import GlobalStyle from "@/src/system/global";

export default function App({ Component, pageProps }: AppProps) {
  return (
    <ThemeProvider theme={theme}>
      <Navbar />
      <NavbarSpacer />
      <GlobalStyle />
      <Component {...pageProps} />
    </ThemeProvider>
  )
}
