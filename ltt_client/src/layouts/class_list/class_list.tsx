import class_list from "./class_list.less"
import {Box, Typography} from "@mui/joy";
import class_icon from '@/assets/svgs/icon_class'
import reuse from '@/assets/reuseable.less'
import Button from "@mui/joy/Button";
export default function ClassList() {

    const arr = [
    ]
    for (let i =0;i<5;i++){
        arr.push(
            <Box
                className={class_list.classlist_bar+" "
                +reuse.row_flex2center_container
            }
                sx={(theme)=>({
                    // flexDirection: 'column',
                    // justifyContent: 'center',
                    // alignContent:'center',
                    color:'black',
                    background:'#ffb6b6'
                })}
                key={i}
                // sx={{width:10,height:10}}
            >
                <div
                    sx={{
                        // opacity:'20%'
                    }}
                    className={reuse.col_flexcenter_container}
                >
                    <svg width="24" height="26" viewBox="0 0 33 36" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M6.03518 28.2958H32.4225V3.64787C32.4225 1.70596 30.8433 0.12674 28.9014 0.12674H6.01405C3.89081 0.12674 0.732361 1.53343 0.732361 5.40843V30.0563C0.732361 33.9313 3.89081 35.338 6.01405 35.338H32.4225V31.8169H6.03518C5.2218 31.7958 4.25349 31.4753 4.25349 30.0563C4.25349 29.8785 4.26933 29.7201 4.29574 29.5757C4.49292 28.5616 5.32391 28.3134 6.03518 28.2958ZM10.676 10.0598C11.4088 9.33636 12.3971 8.9307 13.4269 8.9307C14.4567 8.9307 15.445 9.33636 16.1778 10.0598L16.5721 10.4454L16.9648 10.0598C17.6979 9.33626 18.6865 8.93054 19.7165 8.93054C20.7466 8.93054 21.7352 9.33626 22.4683 10.0598C22.8319 10.4096 23.1212 10.8291 23.3188 11.2933C23.5164 11.7576 23.6183 12.2569 23.6183 12.7614C23.6183 13.266 23.5164 13.7653 23.3188 14.2295C23.1212 14.6937 22.8319 15.1133 22.4683 15.463L16.5739 21.2535L10.6778 15.463C10.3138 15.1135 10.0242 14.6941 9.82627 14.2299C9.62836 13.7658 9.52627 13.2664 9.5261 12.7618C9.52594 12.2572 9.62771 11.7578 9.82531 11.2935C10.0229 10.8292 10.3123 10.4096 10.676 10.0598Z"
                              fill="#ff5050"/>
                    </svg>
                </div>
            </Box>
        )
    }
    return (
        arr
    );
}