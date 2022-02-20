/** @jsx h */
import { h } from 'preact';

import { Summary } from '../components/Summary';

export default {
    title: 'Example/Page',
    component: Summary,
};

const Template = (args) => <Summary {...args} />;

export const WithData = Template.bind({});
WithData.args = {
    data: [{a:1}],
};