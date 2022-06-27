/** @jsx h */
import {h} from 'preact';

import '../static/bootstrap.min.css';
import '../static/shtats.css';
import Tabs from "../components/tabs";
import {generate_commit_data} from "./utils";

export default {
    title: 'Shtats/Tabs',
    component:Tabs,
};

const Template = (args) => <Tabs {...args} >
    <div label={"foo"}>
        <span>Bang</span>
    </div>
    <div label={"bar"}>
        <span>Boom</span>
    </div>
</Tabs>;


export const Default = Template.bind({});
Default.args = {};
