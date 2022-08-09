import yayJpg from '../assets/yay.jpg';
import index_styles from './index.less';
import reuse_styles from '../assets/reuseable.less'
import Headline from "@/layouts/headline/headline";
import ControlPanel from "@/layouts/control_panel/control_panel";
import {Box, CssVarsProvider} from "@mui/joy";
import {GlobalStyles} from '@mui/system';
import type {Theme} from '@mui/joy/styles';
// import  from "@mui/joy/styles/defaultTheme";
import {get_default_theme,bind_style_2_window} from "@/theme/default_theme";
import ClassList from "@/layouts/class_list/class_list";
import CourseList from "@/layouts/course_list/course_list";
import {curstyle} from "@/theme/curtheme";

export default function HomePage() {
    const headheight=curstyle().headheight
    return (
            <Box className={
             reuse_styles.col_flexcontainer}

                 sx={{
                     height: "calc(100vh - 1px - " + headheight + ")"
                 }}
            >
                {/*<div className={index_styles.headline}>*/}

                {/*    */}
                {/*</div>*/}
                {/*<div className={reuse_styles.fillleft_flex + " "*/}
                {/*+ reuse_styles.row_flexcontainer}>*/}
                    {/*<div className={index_styles.sidebar}>*/}
                    {/*    <ClassList></ClassList>*/}
                    {/*</div>*/}
                    <Box className={index_styles.control_panel}
                        sx={{
                            overflowY:"scroll"
                        }}
                    >
                        <ControlPanel></ControlPanel>
                    </Box>
                    <div className={index_styles.main + " " +
                    reuse_styles.fillleft_flex
                    }>

                    </div>
                {/*</div>*/}
            </Box>

    );
}
