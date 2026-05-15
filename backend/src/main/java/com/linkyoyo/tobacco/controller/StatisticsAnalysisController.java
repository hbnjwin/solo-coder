package com.linkyoyo.tobacco.controller;

import com.linkyoyo.tobacco.annotation.SysOperaLog;
import com.linkyoyo.tobacco.entity.SysOperator;
import com.linkyoyo.tobacco.result.R;
import com.linkyoyo.tobacco.service.StatisticsDataService;
import com.linkyoyo.tobacco.util.SysUserUtils;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

import java.util.Map;
import java.util.Objects;

/**
 * 统计分析控制器
 * 提供标准应用分析图表所需的数据接口
 */
@RestController
@RequestMapping("/statistics")
@Slf4j
public class StatisticsAnalysisController {

    @Autowired
    private StatisticsDataService statisticsDataService;

    /**
     * 获取统计分析数据
     * 返回全局统计数据，不按照登录用户的deptId进行分析
     * @return 统计分析数据
     */
    @GetMapping("/data")
    @SysOperaLog(value = "获取统计分析数据", id = "统计分析")
    public R getStatisticsData() {
        // 始终返回全局统计数据，deptId为null
        return statisticsDataService.getStatisticsData(null);
    }
    
    /**
     * 获取指定部门的统计分析数据
     * 管理员可以查看任意部门的统计数据
     * @param deptId 部门ID
     * @return 统计分析数据
     */
    @GetMapping("/data/{deptId}")
    @SysOperaLog(value = "获取部门统计分析数据", id = "统计分析")
    public R getDepartmentStatisticsData(@PathVariable Integer deptId) {
        SysOperator sysOperator = SysUserUtils.currentUser();
        
        if (Objects.nonNull(sysOperator)) {
            // 如果是企业用户，只能查看自己企业的统计数据
            if (sysOperator.getGroupCode().equals("003") && !sysOperator.getDeptId().equals(deptId)) {
                return R.warning("无权限查看其他部门的统计数据");
            }
        }
        
        return statisticsDataService.getStatisticsData(deptId);
    }
    
    /**
     * 初始化模拟数据
     * 仅管理员可操作
     * @return 操作结果
     */
    @PostMapping("/init")
    @SysOperaLog(value = "初始化统计分析模拟数据", id = "统计分析")
    public R initSimulationData() {
        SysOperator sysOperator = SysUserUtils.currentUser();
        
        if (Objects.nonNull(sysOperator)) {
            // 只有管理员可以初始化模拟数据
            if (sysOperator.getGroupCode().equals("003")) {
                return R.warning("无权限执行此操作");
            }
        }
        
        return statisticsDataService.initSimulationData();
    }
    
    /**
     * 获取各地市烟草公司标准文件统计数据
     * @return 各地市烟草公司的标准文件数量统计
     */
    @GetMapping("/standardDocCount")
    @SysOperaLog(value = "获取各地市烟草公司标准文件统计数据", id = "统计分析")
    public R getStandardDocCountByDepartment() {
        try {
            Map<String, Long> docCountByDept = statisticsDataService.countStandardDocsByDepartment();
            return R.ok(docCountByDept);
        } catch (Exception e) {
            log.error("获取各地市烟草公司标准文件统计数据失败", e);
            return R.warning("获取统计数据失败: " + e.getMessage());
        }
    }
}
