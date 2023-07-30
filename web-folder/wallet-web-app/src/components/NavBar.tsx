import { useState } from 'react';
import { IconGauge, IconFingerprint, IconActivity, IconChevronRight, IconHome, IconCreditCard, IconMessageChatbot, IconApps } from '@tabler/icons-react';
import { Box, NavLink } from '@mantine/core';
import Link from "next/link";
import { useRouter } from "next/router";

const data = [
  {
    icon: IconHome,
    label: 'Home',
    description: 'Your Wallet Info',
    rightSection: <IconChevronRight size={16} stroke={1.5} />,
    view: '/'
  },
  {
    icon: IconGauge,
    label: 'Dashboard',
    description: 'Recent Transactions',
    rightSection: <IconChevronRight size={16} stroke={1.5} />,
    view: '/dashboard'
  },
  {
    icon: IconMessageChatbot,
    label: 'Chat',
    description: 'Chat With Our AI Assistant',
    rightSection: <IconChevronRight size={16} stroke={1.5} />,
    view: '/chat'
  },
  {
    icon: IconApps,
    label: 'Mini Apps',
    description: 'Available Mini Apps',
    rightSection: <IconChevronRight size={16} stroke={1.5} />,
  },
  {
    icon: IconCreditCard,
    label: 'Cards',
    description: 'Connected Cards',
    rightSection: <IconChevronRight size={16} stroke={1.5} />,
    view: '/cards'
  },
];

const NavBar = () => {
  const [active, setActive] = useState(0);
  const router = useRouter();

  const items = data.map((item, index) => (
    <Link key={index} href={`${item.view}`} style={{ textDecoration: 'none' }}>
      <NavLink
        style={{ borderRadius: '10px', marginTop: '10px' }}
        key={item.label}
        active={router.pathname === item.view}
        label={item.label}
        description={item.description}
        rightSection={item.rightSection}
        icon={<item.icon size={16} stroke={1.5} />}
        onClick={() => { setActive(index); }}
      /></Link >
  ));

  return <Box>{items}</Box>;
}
export default NavBar;

