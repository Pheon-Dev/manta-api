import { useState } from "react";
import { useRoutes } from "react-router-dom";
import routes from "./routes";
import { IconBrandCodecov } from '@tabler/icons-react';
import {
  AppShell,
  Navbar,
  Header,
  Footer,
  // Aside,
  Text,
  MediaQuery,
  Burger,
  useMantineTheme,
  ActionIcon,
  // Title,
} from '@mantine/core';
import { NavBar, Utilities } from "./components";
// import { getClient, ResponseType } from '@tauri-apps/api/http';
// const client = await getClient();
// const response = await client.get('http://127.0.0.1:8080/send', {
//   timeout: 30,
//   // the expected response type
//   responseType: ResponseType.JSON
// });
// console.log(response);

function App() {
  const element = useRoutes(routes);
  const theme = useMantineTheme();
  const [opened, setOpened] = useState(false);


  return (
    <AppShell
      styles={{
        main: {
          background: theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.colors.gray[0],
        },
      }}
      navbarOffsetBreakpoint="sm"
      asideOffsetBreakpoint="sm"
      navbar={
        <Navbar p="md" hiddenBreakpoint="sm" hidden={!opened} width={{ sm: 250, lg: 300 }}>
          <NavBar />
        </Navbar>
      }
      // aside={
      //   <MediaQuery smallerThan="sm" styles={{ display: 'none' }}>
      //     <Aside p="md" hiddenBreakpoint="sm" width={{ sm: 200, lg: 300 }}>
      //       <Text>Application sidebar</Text>
      //     </Aside>
      //   </MediaQuery>
      // }
      footer={
        <Footer height={60}>
          <Utilities />
        </Footer>
      }
      header={
        <Header height={{ base: 50, md: 70 }} p="md">
          <div style={{ display: 'flex', alignItems: 'center', height: '100%' }}>
            <MediaQuery largerThan="sm" styles={{ display: 'none' }}>
              <Burger
                opened={opened}
                onClick={() => setOpened((o) => !o)}
                size="sm"
                color={theme.colors.gray[6]}
                mr="xl"
              />
            </MediaQuery>
            <ActionIcon variant="gradient" gradient={{ from: 'indigo', to: 'cyan', deg: 45 }} size="md"><IconBrandCodecov size={24} /></ActionIcon>
            <Text
              variant="gradient"
              gradient={{ from: 'indigo', to: 'cyan', deg: 45 }}
              sx={{ fontFamily: 'Greycliff CF, sans-serif' }}
              ta="center" p="xs"
              fz="xl"
              fw={700}
            >{"  "}Manta Wallet</Text>
          </div>
        </Header>
      }
    >
      {element}
    </AppShell>
  );
}

export default App;
