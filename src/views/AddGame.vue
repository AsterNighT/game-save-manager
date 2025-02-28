<script lang="ts" setup>
// !为了不让 submit_handler 报错，这里不使用 lang="ts"
import { DocumentAdd } from "@element-plus/icons-vue";
import {
    Check,
    RefreshRight,
    Download,
} from "@element-plus/icons-vue";
import { reactive, ref } from "vue";
import { useRouter } from "vue-router";
import { useConfig } from "../stores/ConfigFile";
import { invoke } from '@tauri-apps/api/tauri'
import { Game, SaveUnit } from "../schemas/saveTypes";
import { show_error, show_warning } from "../utils/notifications";
import { show_success } from "../utils/notifications";
const router = useRouter();
let config = useConfig();
const buttons = [
    {
        text: "自动识别本地游戏",
        type: "primary",
        icon: Download,
        method: search_local,
    },
    {
        text: "保存当前编辑的配置",
        type: "success",
        icon: Check,
        method: save,
    },
    {
        text: "重置当前配置",
        type: "danger",
        icon: RefreshRight,
        method: reset,
    },
] as const;


const game_name = ref("") // 写入游戏名
let save_paths: Array<SaveUnit> = reactive(new Array<SaveUnit>()) // 选择游戏存档目录
const game_path = ref("") // 选择游戏启动程序
const game_icon_src = ref("https://shadow.elemecdn.com/app/element/hamburger.9cf7b091-55e9-11e9-a976-7f4d0b07eef6.png")

function check_save_unit_unique(p: string) {
    if (save_paths.find((x) => x.path == p)) {
        show_error("该软件暂不支持同名文件/文件夹出现");
        return false;
    }
    return true;
}
function add_save_directory() {
    invoke("choose_save_dir").then((dir) => {
        if (!dir || !check_save_unit_unique(dir as string)) { return }
        let unit: SaveUnit = {
            unit_type: "Folder",
            path: dir as string
        }
        save_paths.push(unit)
    }).catch(
        (e)=>{console.log(e)}
    )
}
function add_save_file() {
    invoke("choose_save_file").then((file) => {
        if (!file || !check_save_unit_unique(file as string)) { return }
        let unit: SaveUnit = {
            unit_type: "File",
            path: file as string
        }
        save_paths.push(unit)
    }).catch(
        (e)=>{console.log(e)}
    )
}
function choose_executable_file() {
    invoke("choose_save_file").then((file) => {
        console.log(file);
        game_path.value = file as string;
    }).catch(
        (e)=>{console.log(e)}
    )
}

function submit_handler(button_method: Function) {
    // 映射按钮的ID和他们要触发的方法
    button_method();
}
function search_local() {
    // TODO:导入已有配置
    show_warning("--WIP-- 这个功能尚未完成");
}
function save() {
    if (game_name.value == "" || save_paths.length == 0) {
        show_error("请至少输入游戏名和一个存档路径");
        return
    }
    let game: Game = {
        name: game_name.value,
        save_paths: save_paths,
        game_path: game_path.value
    }
    invoke("add_game", { game: game }).then((x) => {
        console.log(x);
        reset(false);
        config.refresh();
        show_success("添加成功");
    })
}
function reset(show_notification: boolean = true) {
    // 重置当前配置
    game_name.value = "";
    save_paths = reactive([]);
    game_path.value = "";
    game_icon_src.value =
        "https://shadow.elemecdn.com/app/element/hamburger.9cf7b091-55e9-11e9-a976-7f4d0b07eef6.png";
    if (show_notification) { show_success("重置成功"); }
}

function deleteRow(index: number) {
    save_paths.splice(index, 1);
}
</script>

<template>
    <div class="select-container">
        <el-card class="game-info">
            <div class="top-part">
                <img class="game-icon" :src="game_icon_src" />
                <el-input v-model="game_name" placeholder="请输入游戏名（必须）" class="game-name">
                    <template #prepend>
                        游戏名称
                    </template>
                </el-input>
                <el-input v-model="game_path" placeholder="请选择游戏启动文件（非必须）" class="game-path">
                    <template #prepend>
                        启动文件
                    </template>
                    <template #append>
                        <el-button @click="choose_executable_file()">
                            <el-icon>
                                <document-add />
                            </el-icon>
                        </el-button>
                    </template>
                </el-input>
            </div>
            <div class="add-button-area">
                <el-button type="primary" @click="add_save_directory">添加存档文件夹</el-button>
                <el-button type="primary" @click="add_save_file">添加存档文件</el-button>
            </div>
            <el-table :data="save_paths" class="save-table">
                <el-table-column fixed prop="unit_type" label="类型" width="120" />
                <el-table-column label="控制" width="120">
                    <template #default="scope">
                        <el-button link type="primary" size="small" @click.prevent="deleteRow(scope.$index)">
                            移除
                        </el-button>
                    </template>
                </el-table-column>
                <el-table-column prop="path" label="路径" />
            </el-table>
        </el-card>
        <el-container class="submit-bar">
            <el-tooltip v-for="button in buttons" :key="button.text" :content="button.text" placement="top">
                <el-button @click="submit_handler(button.method)" :type="button.type" circle>
                    <el-icon>
                        <component :is="button.icon" />
                    </el-icon>
                </el-button>
            </el-tooltip>
        </el-container>
    </div>
</template>

<style scoped>
.save-table {
    margin-top: 20px;
    margin-bottom: 20px;
}

.select-container {
    height: 90%;
    width: 100%;
}

.game-info {
    height: 99%;
    margin-bottom: 20px;
}

.top-part {
    height: 200px;
    display: grid;
    grid-template-columns: 1fr 3fr;
    grid-template-rows: 1fr 1fr 1fr 1fr 1fr 1fr;
}

.top-part>img {
    grid-column: 1/2;
    grid-row: 1/7;
}

.game-name {
    grid-column: 2/3;
    grid-row: 5/6;
    margin-bottom: 5px;
}

.game-path {
    grid-column: 2/3;
    grid-row: 6/7;
}

.game-icon {
    float: left;
    height: 200px;
    width: 200px;
}

.add-button-area {
    margin-top: 20px;
}

.submit-bar {
    justify-content: flex-end;
    height: 10%;
}
</style>