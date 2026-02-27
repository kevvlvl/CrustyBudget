import {AppShell, Container, Group, Tabs} from "@mantine/core";
import {Outlet, useLocation, useNavigate} from "react-router-dom";


export function MainLayout() {

    const navigate = useNavigate();
    const location = useLocation();
    const activeTab = location.pathname === '/' ? 'home' : location.pathname.slice(1);

    return (
        <AppShell header={{height: 60}}>
            <AppShell.Header
                p={"md"}
                withBorder={false}>
                <Group>

                    <Tabs value={activeTab} onChange={(value) => navigate(value === 'home' ? '/' : `/${value}`)}>
                        <Tabs.List>
                            <Tabs.Tab value={"home"} >Home</Tabs.Tab>
                            <Tabs.Tab value={"summary"}>Summary</Tabs.Tab>
                            <Tabs.Tab value={"creditdetails"}>Credit Details</Tabs.Tab>
                        </Tabs.List>
                    </Tabs>
                </Group>
            </AppShell.Header>

            <AppShell.Main>

                <Container>
                    <Outlet />
                </Container>

            </AppShell.Main>
        </AppShell>
    );
}