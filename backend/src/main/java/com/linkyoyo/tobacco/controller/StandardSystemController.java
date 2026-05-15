package com.linkyoyo.tobacco.controller;


import com.linkyoyo.tobacco.annotation.SysOperaLog;
import com.linkyoyo.tobacco.info.StandardSystemInfo;
import com.linkyoyo.tobacco.query.StandardSystemQuery;
import com.linkyoyo.tobacco.result.R;
import com.linkyoyo.tobacco.service.StandardSystemService;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.*;

import javax.validation.constraints.NotNull;
import java.lang.reflect.InvocationTargetException;
import java.util.List;

@RequestMapping("/standardSystem")
@RestController
@Validated
public class StandardSystemController {
    @Autowired
    private StandardSystemService standardSystemService ;

    @GetMapping("/list")
    public R getStandardSystemList( StandardSystemQuery standardSystemQuery)
    {
        return  R.ok(standardSystemService.getStandardSystemList(standardSystemQuery)) ;
    }

    @GetMapping("/listDetail")
    public R getStandardSystemDetail(StandardSystemInfo standardSystemInfo)
    {
        return  R.ok(standardSystemService.getStandardSystemDetail(standardSystemInfo.getId())) ;
    }


    @PostMapping ("/update")
    public R updateData(@RequestBody StandardSystemInfo standardSystemInfo)
    {
        return R.ok(standardSystemService.createOrUpdate(standardSystemInfo));
    }

    @GetMapping("/tree")
    public R getTree() throws InvocationTargetException, IllegalAccessException, InstantiationException {
        return R.ok(standardSystemService.getTree()) ;
    }

    @GetMapping("/treeExclude")
    @SysOperaLog(value="获取标准体系树根据公司id",id="标准体系")
    public R getTreeExclude(@NotNull Integer deptId,@NotNull Integer maxLevel)  {
        return standardSystemService.getTreeExclude(deptId,maxLevel) ;
    }


}
