import { useState } from 'react';
import { IconGauge, IconFingerprint, IconActivity, IconChevronRight, IconHome, IconCreditCard } from '@tabler/icons-react';
import { Box, NavLink } from '@mantine/core';
import { NavLink as Link } from 'react-router-dom';

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
    // description: 'Recent Transactions',
    rightSection: <IconChevronRight size={16} stroke={1.5} />,
    view: '/dashboard'
  },
  {
    icon: IconFingerprint,
    label: 'Security',
    // description: 'Safety Measures',

    rightSection: <IconChevronRight size={16} stroke={1.5} />,
  },
  {
    icon: IconActivity,
    label: 'Activity',
    // description: 'Recent Transactions',
    rightSection: <IconChevronRight size={16} stroke={1.5} />,
  },
  {
    icon: IconCreditCard,
    label: 'Cards',
    // description: 'Recent Transactions',
    rightSection: <IconChevronRight size={16} stroke={1.5} />,
    view: '/cards'
  },
];

const NavBar = () => {
  const [active, setActive] = useState(0);

  const items = data.map((item, index) => (
    <Link key={index} to={`${item.view}`} style={{ textDecoration: 'none' }}>
      <NavLink

        style={{ borderRadius: '10px', marginTop: '10px' }}
        key={item.label}
        active={index === active}
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
