package com.linkyoyo.tobacco.service.impl;

import cn.hutool.core.collection.CollectionUtil;
import cn.hutool.core.date.DateUtil;
import cn.hutool.core.io.FileUtil;
import cn.hutool.core.util.ObjectUtil;
import cn.hutool.core.util.StrUtil;
import cn.hutool.json.JSONObject;
import cn.hutool.json.JSONUtil;
import com.github.wenhao.jpa.PredicateBuilder;
import com.github.wenhao.jpa.Specifications;
import com.linkyoyo.tobacco.entity.*;
import com.linkyoyo.tobacco.info.StandardDocInfo;
import com.linkyoyo.tobacco.info.PageInfo;
import com.linkyoyo.tobacco.info.StandardDocDetailResponse;
import com.linkyoyo.tobacco.info.UploadSingleFileInfo;
import com.linkyoyo.tobacco.info.UploadZipFileInfo;
import com.linkyoyo.tobacco.query.StandardDocQuery;
import com.linkyoyo.tobacco.repository.DepartmentDocRepository;
import com.linkyoyo.tobacco.repository.StandardDocRepository;
import com.linkyoyo.tobacco.repository.StandardDocTocRepository;
import com.linkyoyo.tobacco.repository.StandardNormativeReferenceRepository;
import com.linkyoyo.tobacco.repository.StandardSystemRepository;
import com.linkyoyo.tobacco.result.R;
import com.linkyoyo.tobacco.service.StandardDocService;
import com.linkyoyo.tobacco.service.StandardDocTocService;
import com.linkyoyo.tobacco.support.CommonFunc;
import com.linkyoyo.tobacco.util.DeCompressUtil;
import com.linkyoyo.tobacco.util.PageableUtil;
import com.linkyoyo.tobacco.util.StandardPdfAnalyzeService;
import com.linkyoyo.tobacco.util.SysUserUtils;
import com.querydsl.jpa.impl.JPAQueryFactory;
import lombok.extern.slf4j.Slf4j;
import org.apache.poi.hwpf.HWPFDocument;
import org.apache.poi.hwpf.extractor.WordExtractor;
import org.apache.poi.poifs.filesystem.POIFSFileSystem;
import org.apache.poi.ss.usermodel.*;
import org.apache.poi.ss.util.CellRangeAddress;
import org.apache.poi.xssf.usermodel.XSSFWorkbook;

import org.apache.poi.xwpf.usermodel.XWPFDocument;
import org.springframework.beans.BeanUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.stereotype.Service;

import javax.persistence.EntityManager;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.lang.reflect.Field;
import java.math.BigInteger;
import java.util.*;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.time.ZonedDateTime;
import java.util.stream.Collectors;


@Service
@Slf4j
public class StandardDocServiceImpl implements StandardDocService {

    @Value("${paraSet.uploadPath}")
    private String uploadPath;

    @Value("${paraSet.unzipPath}")
    private String unzipPath;

    @Value("${paraSet.standardDocPath}")
    private String standardDocPath;

    @Autowired
    private JPAQueryFactory jpaQueryFactory;

    @Autowired
    private EntityManager entityManager;

    @Autowired
    private StandardDocRepository standardDocRepository;

    @Autowired
    private StandardPdfAnalyzeService standardPdfAnalyzeService;

    @Autowired
    private DepartmentDocRepository departmentDocRepository;

    @Autowired
    private CommonFunc commonFunc;

    @Autowired
    private StandardSystemRepository standardSystemRepository;

    @Autowired
    private StandardNormativeReferenceRepository standardNormativeReferenceRepository;

    @Autowired
    private StandardDocTocRepository standardDocTocRepository;

    @Autowired
    private StandardDocTocService standardDocTocService;

    private void setField(StandardDoc standardDoc, String fieldName, Object fieldValue) {
        try {

            Field field = StandardDoc.class.getDeclaredField(fieldName);
            field.setAccessible(true);
            if (field.isAccessible()) {
                if (field.getType() == Date.class) {
                    try {
                        field.set(standardDoc, DateUtil.parse((String) fieldValue, "yyyy-MM-dd").toJdkDate());
                    } catch (Exception e) {
                        log.error("setField date  error:{}", e.getMessage());
                    }
                } else
                    field.set(standardDoc, fieldValue);
            }
        } catch (NoSuchFieldException | IllegalAccessException e) {
            log.error("setField error:{}", e.getMessage());
        }
    }

    @Override
    public R queryByCondition(StandardDocQuery standardDocQuery) {
        Integer orgPageSize = standardDocQuery.getPageSize();
        standardDocQuery.setPageSize(100000);
        Integer orgPage = standardDocQuery.getPage();
        standardDocQuery.setPage(1);
        Pageable pageable = PageableUtil.build(standardDocQuery);
        PredicateBuilder<StandardDoc> specEnterprise = Specifications.<StandardDoc>and();


        if (standardDocQuery.getSystemId() != null)
            specEnterprise.eq("systemId", standardDocQuery.getSystemId());

        if (standardDocQuery.getEnterpriseId() != null)
            specEnterprise.eq("deptId", standardDocQuery.getEnterpriseId());


        Page<StandardDoc> lstContent =
                standardDocRepository.findAll(
                        Objects.isNull(standardDocQuery.getWhere()) ? specEnterprise.build()
                                : specEnterprise.predicate(CommonFunc.<StandardDoc>getWhere(standardDocQuery)).build()
                        , pageable);

        List<StandardDoc> lst = lstContent.getContent();
        Long noPubCount = lst.stream().filter(f -> StrUtil.isEmpty(f.getStatusLabel()) || f.getStatusLabel().equals("未发布")).count();
        Long effectCount = lst.stream().filter(f -> StrUtil.isNotEmpty(f.getStatusLabel()) && f.getStatusLabel().equals("现行有效")).count();
        int allCount = lst.size();

        if (Objects.nonNull(standardDocQuery.getStatus()) && standardDocQuery.getStatus() != 0) {
            String filterContent = standardDocQuery.getStatus() == 1 ? "未发布" : standardDocQuery.getStatus() == 2 ? "现行有效" : standardDocQuery.getStatus() == 3 ? "已经废止" : "";
            if (StrUtil.isNotEmpty(filterContent))
                lst = lst.stream().filter(f -> f.getStatusLabel().equals(filterContent)).collect(Collectors.toList());

        }
        LinkedHashMap<String, Object> map = new LinkedHashMap<>();


        standardDocQuery.setPageSize(orgPageSize);
        standardDocQuery.setPage(orgPage);

        map.put("noPubCount", noPubCount);
        map.put("effectCount", effectCount);
        map.put("allCount", allCount);
        lstContent = CommonFunc.startPage(lst, PageableUtil.build(standardDocQuery));

//        map.put("data",lstContent.getContent());

        Map<String, Object> mapList = new HashMap<>();
        mapList.put("list", lstContent.getContent());
        mapList.put("total", lst.size());

        map.put("data", mapList);


        return R.ok(map);
    }
    
    @Override
    public R advancedQuery(StandardDocQuery standardDocQuery) {
        // 处理JSON条件字符串
        if (StrUtil.isNotEmpty(standardDocQuery.getCondition())) {
            try {
                JSONObject conditionJson = JSONUtil.parseObj(standardDocQuery.getCondition());
                
                // 解析各个查询条件并设置到standardDocQuery对象中
                if (conditionJson.containsKey("standardLevelName")) {
                    standardDocQuery.setStandardLevelName(conditionJson.getStr("standardLevelName"));
                }
                
                if (conditionJson.containsKey("systemId")) {
                    standardDocQuery.setSystemId(conditionJson.getInt("systemId"));
                }
                
                if (conditionJson.containsKey("standardDeparmentId")) {
                    standardDocQuery.setStandardDeparmentId(conditionJson.getInt("standardDeparmentId"));
                }
                
                if (conditionJson.containsKey("drafter")) {
                    standardDocQuery.setDrafter(conditionJson.getStr("drafter"));
                }
                
                if (conditionJson.containsKey("draftUnit")) {
                    standardDocQuery.setDraftUnit(conditionJson.getStr("draftUnit"));
                }
                
                if (conditionJson.containsKey("publishDate")) {
                    String publishDateStr = conditionJson.getStr("publishDate");
                    if (StrUtil.isNotEmpty(publishDateStr)) {
                        standardDocQuery.setPublishDate(DateUtil.parse(publishDateStr, "yyyy-MM-dd"));
                    }
                }
                
                if (conditionJson.containsKey("implementDate")) {
                    standardDocQuery.setImplementDate(conditionJson.getStr("implementDate"));
                }
                
                if (conditionJson.containsKey("status")) {
                    standardDocQuery.setStatus(conditionJson.getInt("status"));
                }
                
                if (conditionJson.containsKey("standardName")) {
                    standardDocQuery.setStandardName(conditionJson.getStr("standardName"));
                }
                
                if (conditionJson.containsKey("scope")) {
                    standardDocQuery.setScope(conditionJson.getStr("scope"));
                }
            } catch (Exception e) {
                log.error("解析查询条件JSON出错: {}", e.getMessage());
            }
        }
        
        Integer orgPageSize = standardDocQuery.getPageSize();
        standardDocQuery.setPageSize(100000);
        Integer orgPage = standardDocQuery.getPage();
        standardDocQuery.setPage(1);
        Pageable pageable = PageableUtil.build(standardDocQuery);
        PredicateBuilder<StandardDoc> specEnterprise = Specifications.<StandardDoc>and();

        // 构建查询条件
        if (standardDocQuery.getSystemId() != null) {
            specEnterprise.eq("systemId", standardDocQuery.getSystemId());
        }

        if (standardDocQuery.getEnterpriseId() != null) {
            specEnterprise.eq("deptId", standardDocQuery.getEnterpriseId());
        }
        
        // 添加新的查询条件
        if (StrUtil.isNotEmpty(standardDocQuery.getStandardLevelName())) {
            specEnterprise.eq("standardLevelName", standardDocQuery.getStandardLevelName());
        }
        
        if (standardDocQuery.getStandardDeparmentId() != null) {
            specEnterprise.eq("standardDeparmentId", standardDocQuery.getStandardDeparmentId());
        }
        
        if (StrUtil.isNotEmpty(standardDocQuery.getDrafter())) {
            specEnterprise.like("drafter", "%" + standardDocQuery.getDrafter() + "%");
        }
        
        if (StrUtil.isNotEmpty(standardDocQuery.getDraftUnit())) {
            specEnterprise.like("draftUnit", "%" + standardDocQuery.getDraftUnit() + "%");
        }
        
        if (standardDocQuery.getPublishDate() != null) {
            // 获取年月，忽略日
            String yearMonth = DateUtil.format(standardDocQuery.getPublishDate(), "yyyy-MM");
            // 创建当月第一天和下个月第一天的日期作为范围
            Date startDate = DateUtil.parse(yearMonth + "-01", "yyyy-MM-dd");
            Date endDate = DateUtil.offsetMonth(startDate, 1);
            // 使用范围查询替代like查询
            specEnterprise.between("publishDate", startDate, endDate);
        }
        
        if (StrUtil.isNotEmpty(standardDocQuery.getImplementDate())) {
            // 左匹配实施日期
            specEnterprise.like("implementDate", standardDocQuery.getImplementDate() + "%");
        }
        
        if (StrUtil.isNotEmpty(standardDocQuery.getStandardName())) {
            specEnterprise.like("standardName", "%" + standardDocQuery.getStandardName() + "%");
        }
        
        if (StrUtil.isNotEmpty(standardDocQuery.getScope())) {
            specEnterprise.like("scope", "%" + standardDocQuery.getScope() + "%");
        }
        
        // 处理标准状态
        if (Objects.nonNull(standardDocQuery.getStatus())) {
            // 0:未发布, 1:现行有效, -1:已经废止
            if (standardDocQuery.getStatus() == 0) {
                specEnterprise.eq("statusLabel", "未发布");
            } else if (standardDocQuery.getStatus() == 1) {
                specEnterprise.eq("statusLabel", "现行有效");
            } else if (standardDocQuery.getStatus() == -1) {
                specEnterprise.eq("statusLabel", "已经废止");
            }
        }

        // 执行查询
        Page<StandardDoc> lstContent = standardDocRepository.findAll(
                Objects.isNull(standardDocQuery.getWhere()) ? specEnterprise.build()
                        : specEnterprise.predicate(CommonFunc.<StandardDoc>getWhere(standardDocQuery)).build(),
                pageable);

        List<StandardDoc> lst = lstContent.getContent();
        Long noPubCount = lst.stream().filter(f -> StrUtil.isEmpty(f.getStatusLabel()) || f.getStatusLabel().equals("未发布")).count();
        Long effectCount = lst.stream().filter(f -> StrUtil.isNotEmpty(f.getStatusLabel()) && f.getStatusLabel().equals("现行有效")).count();
        int allCount = lst.size();

        // 恢复原始分页参数
        standardDocQuery.setPageSize(orgPageSize);
        standardDocQuery.setPage(orgPage);
        
        // 构建返回结果
        LinkedHashMap<String, Object> map = new LinkedHashMap<>();
        map.put("noPubCount", noPubCount);
        map.put("effectCount", effectCount);
        map.put("allCount", allCount);
        
        // 分页处理
        lstContent = CommonFunc.startPage(lst, PageableUtil.build(standardDocQuery));

        Map<String, Object> mapList = new HashMap<>();
        mapList.put("list", lstContent.getContent());
        mapList.put("total", lst.size());
        map.put("data", mapList);

        return R.ok(map);
    }


    @Override
    public PageInfo<StandardDoc> getListByEnterpriseAll(StandardDocQuery standardDocQuery) {
        Pageable pageable = PageableUtil.build(standardDocQuery);
        PredicateBuilder<StandardDoc> specEnterprise = Specifications.<StandardDoc>and();

//        specEnterprise.eq("status", "1");

        if (standardDocQuery.getSystemId() != null)
            specEnterprise.eq("systemId", standardDocQuery.getSystemId());

        if (standardDocQuery.getEnterpriseId() != null)
            specEnterprise.eq("deptId", standardDocQuery.getEnterpriseId());


        return PageableUtil.info(standardDocRepository.findAll(
                Objects.isNull(standardDocQuery.getWhere()) ? specEnterprise.build()
                        : specEnterprise.predicate(CommonFunc.<StandardDoc>getWhere(standardDocQuery)).build()
                , pageable));

    }

    @Override
    public PageInfo<StandardDoc> getListByEnterprise(StandardDocQuery standardDocQuery) {
        Pageable pageable = PageableUtil.build(standardDocQuery);
        PredicateBuilder<StandardDoc> specEnterprise = Specifications.<StandardDoc>and();

        specEnterprise.eq("status", "1");

        if (standardDocQuery.getSystemId() != null)
            specEnterprise.eq("systemId", standardDocQuery.getSystemId());

        if (standardDocQuery.getEnterpriseId() != null)
            specEnterprise.eq("deptId", standardDocQuery.getEnterpriseId());

        specEnterprise.eq("delFlag", false);


        return PageableUtil.info(standardDocRepository.findAll(
                Objects.isNull(standardDocQuery.getWhere()) ? specEnterprise.build()
                        : specEnterprise.predicate(CommonFunc.<StandardDoc>getWhere(standardDocQuery)).build()
                , pageable));

    }


    @Override
    public PageInfo<StandardDoc> getStandardDocList(StandardDocQuery standardDocQuery) {
        Pageable pageable = PageableUtil.build(standardDocQuery);
        PredicateBuilder<StandardDoc> specEnterprise = Specifications.<StandardDoc>and();


        if (standardDocQuery.getSystemId() != null)
            specEnterprise.eq("systemId", standardDocQuery.getSystemId());

        if (standardDocQuery.getEnterpriseId() != null)
            specEnterprise.eq("deptId", standardDocQuery.getEnterpriseId());

        specEnterprise.eq("delFlag", false);


        return PageableUtil.info(standardDocRepository.findAll(
                Objects.isNull(standardDocQuery.getWhere()) ? specEnterprise.build()
                        : specEnterprise.predicate(CommonFunc.<StandardDoc>getWhere(standardDocQuery)).build()
                , pageable));

//        return PageableUtil.info(standardDocRepository.findAll(CommonFunc.<StandardDoc>getWhere(standardDocQuery), pageable));
    }


    @Override
    public File getFile(Integer id) {
        QStandardDoc qStandardDoc = QStandardDoc.standardDoc;
//        StandardDoc standardDoc = jpaQueryFactory.selectFrom(qStandardDoc).where(qStandardDoc.id.eq(id).and(qStandardDoc.status.eq(1))).fetchOne();
        StandardDoc standardDoc = jpaQueryFactory.selectFrom(qStandardDoc).where(qStandardDoc.id.eq(id)).fetchOne();
        if (Objects.isNull(standardDoc))
            return null;
        else {
//            log.info("standardDoc.getFileUrl():" + standardDoc.getFileUrl()+"##");
            if (!FileUtil.exist(standardDocPath + File.separator.concat(Objects.isNull(standardDoc.getDeptId()) ? "all" : standardDoc.getDeptId().toString()) + File.separator + standardDoc.getFileUrl().trim())) {
                log.info("目录:{},文件名字:{} 不存在！", standardDocPath, standardDoc.getFileUrl());
                return null;
            }

            File file = FileUtil.touch(standardDocPath + File.separator.concat(Objects.isNull(standardDoc.getDeptId()) ? "all" : standardDoc.getDeptId().toString()), standardDoc.getFileUrl().trim());
            if (!file.exists())
                return null;
            else
                return file;
        }

    }


    @Override
    public File getSumFile(Integer deptId) {
        DepartmentDoc departmentDoc = departmentDocRepository.findById(deptId).orElse(null);
        if (Objects.isNull(departmentDoc))
            return null;

        List list = commonFunc.commQuerySql(

                String.format(
                        "select  a.system_name,COALESCE(b.gb_count,0) gb_count,COALESCE(b.hy_count,0) hy_count,COALESCE(b.zgs_count,0) zgs_count,COALESCE(b.sgs_count,0) sgs_count,COALESCE(b.fgs_count,0) fgs_count,COALESCE(b.fj_count,0) fj_count,COALESCE(b.hj,0) hj from standard_system a   \n" +
                                "left join ( \n" +
                                "select  max(system_name) as system_name,\n" +
                                "       sum( case  when  standard_level_name='GB'  then 1 else 0 end) as gb_count ,\n" +
                                "\t\t\t sum( case  when  standard_level_name='YC' then 1 else 0 end) as hy_count ,\n" +
                                "       sum( case  when  standard_level_name='YQ'  then 1 else 0 end) as zgs_count ,\n" +
                                "       sum( case  when  standard_level_name='HNYQ' or standard_level_name='Q/HNYC' or standard_level_name='HNYC' then 1 else 0 end) as sgs_count ,\n" +
                                "       sum( case  when  standard_level_name='%s'  then 1 else 0 end) as fgs_count,\n" +
                                "\t\t\t count(0) hj,\n" +
                                "\t\t\t sum( case  when  standard_level_name='GB' or standard_level_name='YC' or standard_level_name='YQ' or standard_level_name='HNYQ' or standard_level_name='HNYC' or standard_level_name='Q/HNYC' or standard_level_name='%s'  then 1 else 0 end) as fj_count \n" +
                                "  from standard_doc\n" +
                                "\twhere not del_flag and status=1 and dept_id=%d  \n" +
                                "group by system_id " +
                                ")b  on a.system_name=b.system_name \n" +
                                " where system_level<=1 "
//                        and exclude_dept_id not like '\%," +deptId.toString()+",\% "
                        , departmentDoc.getYqbz(), departmentDoc.getYqbz(), deptId));
//        .concat(" and exclude_dept_id not like '%," + deptId.toString() + ",%' ")

        if (list.size() == 0)
            return null;

        // 创建Excel工作簿
        Workbook workbook = new XSSFWorkbook();
        // 创建一个工作表sheet
        Sheet sheet = workbook.createSheet("商业企业标准统计表");


        CellStyle titleStyle = workbook.createCellStyle();
        titleStyle.setFillForegroundColor(IndexedColors.WHITE.getIndex());
        titleStyle.setFillPattern(FillPatternType.SOLID_FOREGROUND);
        titleStyle.setAlignment(HorizontalAlignment.CENTER);
        titleStyle.setVerticalAlignment(VerticalAlignment.CENTER);
        Font titleFont = workbook.createFont();
        titleFont.setFontHeightInPoints((short) 15);
        titleFont.setBold(true);
        titleStyle.setFont(titleFont);

        Row firstRow = sheet.createRow(0);
        CellRangeAddress cellRangeAddress = new CellRangeAddress(0, 0, 0, deptId == 5 ? 5 : 6); // 合并第一行的第一个单元格到第三个单元格
        sheet.addMergedRegion(cellRangeAddress);

        Cell firstCell = firstRow.createCell(0);
        firstCell.setCellValue(departmentDoc.getDeptName().concat("商业企业标准统计表"));
        firstCell.setCellStyle(titleStyle);


//        Row titleRow = sheet.createRow(1);
        // 设置表头背景色

        // 设置表头样式
        CellStyle headerStyle = workbook.createCellStyle();
        headerStyle.setFillForegroundColor(IndexedColors.LIGHT_BLUE.getIndex());
        headerStyle.setFillPattern(FillPatternType.SOLID_FOREGROUND);
        headerStyle.setAlignment(HorizontalAlignment.CENTER);
        headerStyle.setVerticalAlignment(VerticalAlignment.CENTER);

        headerStyle.setBorderTop(BorderStyle.THIN);
        headerStyle.setBorderBottom(BorderStyle.THIN);
        headerStyle.setBorderLeft(BorderStyle.THIN);
        headerStyle.setBorderRight(BorderStyle.THIN);

        Font headerFont = workbook.createFont();
        headerFont.setBold(true);
        headerFont.setColor(IndexedColors.WHITE.getIndex());
        headerStyle.setFont(headerFont);

        // 设置数据行样式
        CellStyle rowStyle = workbook.createCellStyle();
        rowStyle.setBorderTop(BorderStyle.THIN);
        rowStyle.setBorderBottom(BorderStyle.THIN);
        rowStyle.setBorderLeft(BorderStyle.THIN);
        rowStyle.setBorderRight(BorderStyle.THIN);
        Font rowFont = workbook.createFont();
        rowFont.setFontHeightInPoints((short) 10);
        rowStyle.setFont(rowFont);

        CellStyle style = workbook.createCellStyle();
        style.setFillBackgroundColor(IndexedColors.LIGHT_BLUE.getIndex());
        style.setFillForegroundColor(IndexedColors.BLACK.getIndex()); // 设置前景填充颜色为黄色
        style.setFillPattern(FillPatternType.SOLID_FOREGROUND); // 设置前景填充样式

        String[] title = deptId == 5 ? new String[]{"system_name", "gb_count", "hy_count", "zgs_count", "sgs_count", "fj_count"} :
                new String[]{"system_name", "gb_count", "hy_count", "zgs_count", "sgs_count", "fgs_count", "fj_count"};
        String[] titleCn = deptId == 5 ? new String[]{"子体系", "国家标准GB", "行业标准YC", "总公司标准YQ", "省级标准Q/HNYC", "合计"} :
                new String[]{"子体系", "国家标准GB", "行业标准YC", "总公司标准YQ", "省级标准Q/HNYC", departmentDoc.getDeptName().concat(" ").concat(departmentDoc.getYqbz()), "合计"};
        Row headerRow = sheet.createRow(sheet.getLastRowNum() + 1);
        Integer index = 0;
        for (String f : titleCn) {
            Cell cell = headerRow.createCell(index);
            cell.setCellValue(f);
            cell.setCellStyle(headerStyle);
            index++;
        }

        sheet.setColumnWidth(0, 20 * 256);
        sheet.setColumnWidth(1, 21 * 256);
        sheet.setColumnWidth(2, 21 * 256);
        sheet.setColumnWidth(3, 21 * 256);
        sheet.setColumnWidth(4, 21 * 256);
        sheet.setColumnWidth(5, 35 * 256);


        Map<String, Integer> totalMap = new HashMap<>();
        for (Object obj : list) {
            Map<String, Object> orgMap = (Map<String, Object>) obj;
            Row dataRow = sheet.createRow(sheet.getLastRowNum() + 1);
            index = 0;
            for (String f : title) {
                Cell cell = dataRow.createCell(index);
                if (f.equals("system_name"))
                    cell.setCellValue((String) orgMap.get(f));
                else {
                    if (Objects.nonNull(totalMap.get(f))) {
                        totalMap.put(f, totalMap.get(f) + ((BigInteger) orgMap.get(f)).intValue());
                    } else {
                        totalMap.put(f, ((BigInteger) orgMap.get(f)).intValue());
                    }

                    cell.setCellValue(((BigInteger) orgMap.get(f)).intValue());
                }

                cell.setCellStyle(rowStyle);
                index++;
            }

//            map.put("first_target_name", (String) orgMap.get("first_target_name"));
        }


        Row totalRow = sheet.createRow(sheet.getLastRowNum() + 1);
        index = 0;
        for (String f : title) {
            Cell cell = totalRow.createCell(index);
            if (f.equals("system_name"))
                cell.setCellValue("总计");
            else {
                cell.setCellValue(totalMap.get(f));
            }

            cell.setCellStyle(rowStyle);
            index++;
        }


        // 保存Excel文件
        File file = new File("standardTotal" + System.currentTimeMillis() + ".xlsx");
        log.info("导出Excel文件路径:{}", file.getAbsolutePath());
        try (FileOutputStream outputStream = new FileOutputStream(file)) {
            workbook.write(outputStream);
            return file;
        } catch (Exception e) {
            log.error("导出Excel文件出错:{}", e.getMessage());
            return null;
        }

    }

    @Override
    public File getDetailFile(Integer deptId, Integer systemId) {
        DepartmentDoc departmentDoc = departmentDocRepository.findById(deptId).orElse(null);
        if (Objects.isNull(departmentDoc))
            return null;


        StandardSystem standardSystem = standardSystemRepository.getOne(systemId);
        if (standardSystem == null)
            return null;

        List<StandardDoc> list = standardDocRepository.findAll(Specifications.<StandardDoc>and()
                .eq("deptId", deptId).eq("systemId", systemId).eq("delFlag", false)
                .eq("status", 1).build());
//        if (Objects.isNull(list) || list.size()==0)
//            return null;
        if (list.size() > 0)
            CollectionUtil.sort(list, (o1, o2) -> o1.getStandardNo().compareToIgnoreCase(o2.getStandardNo()));

        // 创建Excel工作簿
        Workbook workbook = new XSSFWorkbook();
        // 创建一个工作表sheet
        Sheet sheet = workbook.createSheet("标准明细表");


        CellStyle titleStyle = workbook.createCellStyle();
        titleStyle.setFillForegroundColor(IndexedColors.WHITE.getIndex());
        titleStyle.setFillPattern(FillPatternType.SOLID_FOREGROUND);
        titleStyle.setAlignment(HorizontalAlignment.CENTER);
        titleStyle.setVerticalAlignment(VerticalAlignment.CENTER);
        Font titleFont = workbook.createFont();
        titleFont.setFontHeightInPoints((short) 15);
        titleFont.setBold(true);
        titleStyle.setFont(titleFont);

        Row firstRow = sheet.createRow(0);
        CellRangeAddress cellRangeAddress = new CellRangeAddress(0, 0, 0, 3); // 合并第一行的第一个单元格到第三个单元格
        sheet.addMergedRegion(cellRangeAddress);

        Cell firstCell = firstRow.createCell(0);
//        list.get(0).getSystemName()
        firstCell.setCellValue(departmentDoc.getDeptName().concat("-").concat(standardSystem.getSystemName()).concat(" 标准明细表"));
        firstCell.setCellStyle(titleStyle);


//        Row titleRow = sheet.createRow(1);
        // 设置表头背景色

        // 设置表头样式
        CellStyle headerStyle = workbook.createCellStyle();
        headerStyle.setFillForegroundColor(IndexedColors.LIGHT_BLUE.getIndex());
        headerStyle.setFillPattern(FillPatternType.SOLID_FOREGROUND);
        headerStyle.setAlignment(HorizontalAlignment.CENTER);
        headerStyle.setVerticalAlignment(VerticalAlignment.CENTER);

        headerStyle.setBorderTop(BorderStyle.THIN);
        headerStyle.setBorderBottom(BorderStyle.THIN);
        headerStyle.setBorderLeft(BorderStyle.THIN);
        headerStyle.setBorderRight(BorderStyle.THIN);

        Font headerFont = workbook.createFont();
        headerFont.setBold(true);
        headerFont.setColor(IndexedColors.WHITE.getIndex());
        headerStyle.setFont(headerFont);

        // 设置数据行样式
        CellStyle rowStyle = workbook.createCellStyle();
        rowStyle.setBorderTop(BorderStyle.THIN);
        rowStyle.setBorderBottom(BorderStyle.THIN);
        rowStyle.setBorderLeft(BorderStyle.THIN);
        rowStyle.setBorderRight(BorderStyle.THIN);
        Font rowFont = workbook.createFont();
        rowFont.setFontHeightInPoints((short) 10);
        rowStyle.setFont(rowFont);

        CellStyle style = workbook.createCellStyle();
        style.setFillBackgroundColor(IndexedColors.LIGHT_BLUE.getIndex());
        style.setFillForegroundColor(IndexedColors.BLACK.getIndex()); // 设置前景填充颜色为黄色
        style.setFillPattern(FillPatternType.SOLID_FOREGROUND); // 设置前景填充样式

        String[] title = {"xh", "standardName", "standardNo", "deptName"};
        String[] titleCn = {"序号", "标准名称", "标准编号", "归口部门"};
        Row headerRow = sheet.createRow(sheet.getLastRowNum() + 1);

        Integer index = 0;
        for (String f : titleCn) {
            Cell cell = headerRow.createCell(index);
            cell.setCellValue(f);
            cell.setCellStyle(headerStyle);
            index++;
        }

        sheet.setColumnWidth(0, 10 * 256);
        sheet.setColumnWidth(1, 35 * 256);
        sheet.setColumnWidth(2, 22 * 256);
        sheet.setColumnWidth(3, 22 * 256);


        Map<String, Integer> totalMap = new HashMap<>();
        index = 1;
        for (StandardDoc obj : list) {

            Row dataRow = sheet.createRow(sheet.getLastRowNum() + 1);


            Cell cell0 = dataRow.createCell(0);
            cell0.setCellValue(index);
            cell0.setCellStyle(rowStyle);

            Cell cell1 = dataRow.createCell(1);
            cell1.setCellValue(obj.getStandardName());
            cell1.setCellStyle(rowStyle);


            Cell cell2 = dataRow.createCell(2);
            cell2.setCellValue(obj.getStandardNo());
            cell2.setCellStyle(rowStyle);

            Cell cell3 = dataRow.createCell(3);
            cell3.setCellValue(obj.getStandardDeparmentName());
            cell3.setCellStyle(rowStyle);
            index++;

        }


        // 保存Excel文件
        File file = new File("standardDetail" + System.currentTimeMillis() + ".xlsx");
        log.info("导出Excel文件路径:{}", file.getAbsolutePath());
        try (FileOutputStream outputStream = new FileOutputStream(file)) {
            workbook.write(outputStream);
            return file;
        } catch (Exception e) {
            log.error("导出Excel文件出错:{}", e.getMessage());
            return null;
        }

    }

    @Override
    public R uploadZip(UploadZipFileInfo uploadZipInfo) {

        String fileName = uploadZipInfo.getFile().getOriginalFilename();
        String zipName = String.format("%s.%s", FileUtil.mainName(fileName), FileUtil.extName(fileName));  //System.currentTimeMillis(),

        File zipFile = FileUtil.touch(unzipPath, zipName);
        QSysOperator qSysOperator = QSysOperator.sysOperator;
        QStandardSystem qStandardSystem = QStandardSystem.standardSystem;
        CommonFunc.clearEntityManager(entityManager);

        StandardSystem standardSystem = jpaQueryFactory.selectFrom(qStandardSystem).where(qStandardSystem.id.eq(uploadZipInfo.getSystemId())).fetchOne();

        QStandardDepartment qStandardDepartment = QStandardDepartment.standardDepartment;
        StandardDepartment standardDepartment = Objects.nonNull(uploadZipInfo.getStandardDeparmentId()) ?
                jpaQueryFactory.selectFrom(qStandardDepartment).where(qStandardDepartment.id.eq(uploadZipInfo.getStandardDeparmentId())).fetchOne() : null;


        SysOperator sysUser = jpaQueryFactory.select(qSysOperator).from(qSysOperator).where(qSysOperator.id.eq(uploadZipInfo.getUserId())).fetchOne();
//                ObjectUtil.isEmpty(uploadZipInfo.getUserId()) ? jpaQueryFactory.select(qSysOperator).from(qSysOperator).where(qSysOperator.mobilePhone.eq(uploadZipInfo.getMobilePhone())).fetchOne() :
//                jpaQueryFactory.select(qSysOperator).from(qSysOperator).where(qSysOperator.id.eq(uploadZipInfo.getUserId())).fetchOne();

        try {
            log.info("文件保存到: {}---{}", zipFile.getAbsolutePath(), zipFile.getName());
            uploadZipInfo.getFile().transferTo(zipFile);
            File extractFilePath = FileUtil.file(unzipPath, String.valueOf(System.currentTimeMillis()));
            // File extractFile = FileUtil.touch(extractFilePath, zipName);
//                    FileUtil.file(standardDocPath);
            if (!FileUtil.exist(extractFilePath))
                FileUtil.mkdir(extractFilePath);

            //FileUtil.del(extractFilePath);

            //获取标准文件命名参数
            QSysParaset qSysParaset = QSysParaset.sysParaset;
            SysParaset sysParaset = jpaQueryFactory.select(qSysParaset).from(qSysParaset).fetchFirst();
            JSONObject jsonObject = JSONUtil.parseObj(sysParaset.getFileTitleFormat());
            String separate = jsonObject.getStr("separate");
            String[] fields = jsonObject.getStr("fields").split(",");

            DeCompressUtil.deCompress(zipFile, extractFilePath);
            log.info("解压缩目录: {}", extractFilePath);
            HashMap<String, Object> map = new HashMap<>();

            List<StandardDoc> lstStandardDoc = new ArrayList<>();


//            FileUtil.loopFiles(extractFilePath).forEach(file ->
            for (File file :FileUtil.loopFiles(extractFilePath))
            {
                QStandardDoc qStandardDoc = QStandardDoc.standardDoc;
//                StandardDoc standardDoc = jpaQueryFactory.select(qStandardDoc).from(qStandardDoc).where(qStandardDoc.fileUrl.eq(file.getName())).fetchFirst();
//                Objects.isNull(standardDoc)
                if (1==1) {

//                    拷贝到标准文档目录
                    if (!FileUtil.exist(standardDocPath + File.separator.concat(Objects.isNull(sysUser.getDeptId()) ? "all" : sysUser.getDeptId().toString())))
                        FileUtil.mkdir(standardDocPath + File.separator.concat(Objects.isNull(sysUser.getDeptId()) ? "all" : sysUser.getDeptId().toString()));
                    FileUtil.copy(file, new File(standardDocPath + File.separator.concat(Objects.isNull(sysUser.getDeptId()) ? "all" : sysUser.getDeptId().toString()) + File.separator + file.getName()), true);
                    JSONObject jsonPhrase = standardPdfAnalyzeService.analyze(file, sysParaset.getUseOcr(), sysParaset.getUseLlm(), sysParaset.getDpi(), sysParaset);

                    if (Objects.nonNull(jsonPhrase) && jsonPhrase.size() > 0) {

                        if (jsonPhrase.containsKey("standardNo")) {
                            String standardNo =  jsonPhrase.get("standardNo").toString();
                            CommonFunc.clearEntityManager(entityManager);

                            StandardDoc standardDoc1 = jpaQueryFactory.select(qStandardDoc).from(qStandardDoc).where(qStandardDoc.standardNo.eq(standardNo)
                                    .and(qStandardDoc.deptId.eq(sysUser.getDeptId()))).fetchFirst();
                            if (Objects.nonNull(standardDoc1))
                                continue;
//                                return R.warning("该标准已经上传，请核对！");
                        }
                    }

                    String[] title = FileUtil.mainName(file.getName()).split(separate);


                    StandardDoc standardDoc = StandardDoc.builder()
                            .delFlag(false)
                            .status(0).statusLabel("未发布")
                            .standardName(FileUtil.mainName(file.getName()))
                            .fileUrl(file.getName())
                            .groupName(sysUser.getGroupCode())
                            .deptId(sysUser.getDeptId())
                            .deptName(sysUser.getDeptName()).operatorCode(sysUser.getOperatorCode()).creater(sysUser.getOperatorName())
                            .mobilePhone(sysUser.getMobilePhone())
                            .createDate(new Date())
                            .systemId(uploadZipInfo.getSystemId())
                            .systemName(Objects.nonNull(standardSystem) ? standardSystem.getSystemName() : null)
                            .standardDeparmentId(uploadZipInfo.getStandardDeparmentId())
                            .standardDeparmentName(Objects.nonNull(standardDepartment) ? standardDepartment.getStandardDepartmentName() : null)
                            .remark(StrUtil.format("文件保存到: {}---{}", zipFile.getAbsolutePath(), zipFile.getName()))
                            .build();

                    if (Objects.nonNull(jsonPhrase) && jsonPhrase.size() > 0) {
                        try {


                            if (jsonPhrase.containsKey("standardLevelName"))
                                standardDoc.setStandardLevelName(jsonPhrase.get("standardLevelName").toString());

                            if (jsonPhrase.containsKey("standardNo"))
                                standardDoc.setStandardNo(jsonPhrase.get("standardNo").toString());

                            if (jsonPhrase.containsKey("standardName"))
                                standardDoc.setStandardName(jsonPhrase.get("standardName").toString());


                            if (jsonPhrase.containsKey("publishUnit"))
                                standardDoc.setPublishUnit(jsonPhrase.get("publishUnit").toString());

                            if (jsonPhrase.containsKey("publishDate"))
                                standardDoc.setPublishDate(DateUtil.parse(jsonPhrase.get("publishDate").toString()));
                        } catch (Exception e) {
                            log.error("解析json出错: {}", e.getMessage());
                        }

                    } else {

                        if (title.length == fields.length) {
                            for (int i = 0; i < fields.length; i++) {
                                setField(standardDoc, fields[i], title[i]);
                            }
                        }
                    }
                    standardDocRepository.save(standardDoc);
                    lstStandardDoc.add(standardDoc);

//                  map.put(file.getName(), standardDoc) ;
                }

            }
            //);
            Map<String, Object> mapFiles = new HashMap<>();
            mapFiles.put("uploadSuccessFiles", lstStandardDoc.size());
            mapFiles.put("standardSystemId", Objects.nonNull(standardSystem) ? standardSystem.getId() : null);
            mapFiles.put("lstStandardDoc", lstStandardDoc);
            return R.ok(mapFiles);
//            return R.ok(lstStandardDoc);
        } catch (IOException e) {
            log.error("文件保存到磁盘异常: {}", zipFile.getAbsolutePath(), e);
            return R.warning("文件保存到磁盘异常");
        }

//        deviceDataService.uploadZip(uploadZipInfo);

//        return R.ok("文件上传成功");

    }


    private File convertToPdf(File originFile) {
        String extName = FileUtil.extName(originFile);
        if (!extName.toLowerCase().equals("doc") && !extName.toLowerCase().equals("docx"))
            return originFile;
        String path = FileUtil.getParent(FileUtil.getAbsolutePath(originFile), 1);
        String mainName = FileUtil.mainName(originFile);
        File pdfFile = new File(path + File.separator + mainName + ".pdf");


        if (extName.equals("docx")) {

            try {
                FileOutputStream out = new FileOutputStream(path + File.separator + mainName + ".pdf");
                XWPFDocument document = new XWPFDocument(new FileInputStream(originFile.getAbsolutePath()));

                // 转换为PDF
               /* PdfOptions options = PdfOptions.create();
                PdfConverter.getInstance().convert(document, out, options);*/

                // 关闭文档
                document.close();
                out.close();
                return pdfFile;

            } catch (Exception e) {
                log.error("转换错误:", e.getMessage());
                return originFile;
            }
        } else {

            try {


        /*        fs = new POIFSFileSystem(new FileInputStream(originFile.getAbsolutePath()));

                HWPFDocument doc = new HWPFDocument(fs);*/

                FileOutputStream out = new FileOutputStream(path + File.separator + mainName + ".pdf");
                HWPFDocument document = new HWPFDocument(new POIFSFileSystem(new FileInputStream(originFile.getAbsolutePath())));


                WordExtractor wd = new WordExtractor(document);

                String text = wd.getText();

                // 转换为PDF

              /*   WordToPdfConverter converter = new WordToPdfConverter(DocumentBuilderFactory.newInstance().newDocumentBuilder().newDocument());
                converter.setPdfAuthor("Your Name");
                converter.setPdfTitle("Document Title");
                converter.convert(wordDocument, new FileOutputStream(pdfFilePath));*/
               /* Document pdf= new Document(PageSize.A4);

                PdfWriter.getInstance(pdf, out);

                pdf.open();
                pdf.add(new Paragraph(text));*/

                // 关闭文档
                document.close();
                out.close();
                return pdfFile;

            } catch (Exception e) {
                log.error("转换错误:", e.getMessage());
                return originFile;
            }

        }


    }

    @Override
    public R uploadFile(UploadSingleFileInfo uploadZipInfo) throws IOException {
        String fileName = uploadZipInfo.getFile().getOriginalFilename();
        String zipName = String.format("%s.%s", FileUtil.mainName(fileName), FileUtil.extName(fileName));  //System.currentTimeMillis(),


        QSysOperator qSysOperator = QSysOperator.sysOperator;

        QStandardSystem qStandardSystem = QStandardSystem.standardSystem;
        CommonFunc.clearEntityManager(entityManager);

        StandardSystem standardSystem = jpaQueryFactory.selectFrom(qStandardSystem).where(qStandardSystem.id.eq(uploadZipInfo.getSystemId())).fetchOne();

        QStandardDepartment qStandardDepartment = QStandardDepartment.standardDepartment;
        StandardDepartment standardDepartment = Objects.nonNull(uploadZipInfo.getStandardDeparmentId()) ?
                jpaQueryFactory.selectFrom(qStandardDepartment).where(qStandardDepartment.id.eq(uploadZipInfo.getStandardDeparmentId())).fetchOne() : null;

        SysOperator sysUser = jpaQueryFactory.select(qSysOperator).from(qSysOperator).where(qSysOperator.id.eq(uploadZipInfo.getUserId())).fetchOne();


        //获取标准文件命名参数
        QSysParaset qSysParaset = QSysParaset.sysParaset;
        SysParaset sysParaset = jpaQueryFactory.select(qSysParaset).from(qSysParaset).fetchFirst();

//        File zipFile = FileUtil.touch(standardDocPath, zipName);
        File zipFile = FileUtil.touch(standardDocPath + File.separator.concat(Objects.isNull(sysUser.getDeptId()) ? "all" : sysUser.getDeptId().toString()), zipName);
        try {

            if (!FileUtil.exist(standardDocPath + File.separator.concat(Objects.isNull(sysUser.getDeptId()) ? "all" : sysUser.getDeptId().toString())))
                FileUtil.mkdir(standardDocPath + File.separator.concat(Objects.isNull(sysUser.getDeptId()) ? "all" : sysUser.getDeptId().toString()));

            uploadZipInfo.getFile().transferTo(zipFile);

//            File newFile = convertToPdf(zipFile) ;

            JSONObject jsonPhrase = standardPdfAnalyzeService.analyze(zipFile, sysParaset.getUseOcr(), sysParaset.getUseLlm(), sysParaset.getDpi(), sysParaset);

            if (jsonPhrase == null)
                return R.warning("解析失败");
            QStandardDoc qStandardDoc = QStandardDoc.standardDoc;


            StandardDoc standardDoc = ObjectUtil.isEmpty(uploadZipInfo.getId()) ? StandardDoc.builder()
                    .delFlag(false)
                    .status(0).statusLabel("未发布")
                    .standardName(FileUtil.mainName(zipFile.getName()))
                    .fileUrl(zipFile.getName())
                    .groupName(sysUser.getGroupCode())
                    .deptId(sysUser.getDeptId())
                    .deptName(sysUser.getDeptName()).operatorCode(sysUser.getOperatorCode())
                    .mobilePhone(sysUser.getMobilePhone())
                    .systemId(uploadZipInfo.getSystemId())
                    .systemName(Objects.nonNull(standardSystem) ? standardSystem.getSystemName() : null)
                    .standardDeparmentId(uploadZipInfo.getStandardDeparmentId())
                    .standardDeparmentName(Objects.nonNull(standardDepartment) ? standardDepartment.getStandardDepartmentName() : null)
                    .createDate(new Date()).creater(sysUser.getOperatorName()).createrId(sysUser.getId())
                    .remark(StrUtil.format("文件保存到: {}---{}", zipFile.getAbsolutePath(), zipFile.getName()))
                    .build()
                    : jpaQueryFactory.select(qStandardDoc).from(qStandardDoc).where(qStandardDoc.id.eq(uploadZipInfo.getId())).fetchOne();


            standardDoc.setFileUrl(zipFile.getName());
            //pdf内容解析 && jsonPhrase.containsKey("standardNo")
            if (Objects.nonNull(jsonPhrase) && jsonPhrase.size() > 0) {
                try {
                    // 创建final副本以符合lambda表达式的要求
                    final JSONObject finalJsonPhrase = jsonPhrase;
                    final StandardDoc finalStandardDoc = standardDoc;

                    // 对每个字段进行处理
                    Arrays.asList("publishDate", "standardLevelName", "standardNo", "standardName", "publishUnit",
                               "standardStatus","aiAnalysisResult","implementDate", "standardEnName", "draftUnit", "drafter", "proposeUnit",
                               "scope", "introduction", "referenceDocs", "normativeReferences", "termsAndDefinitions", "preface","simpleMdFilePath"
                    ,"mdFilePath").forEach(f -> {
                        try {
                            if (finalJsonPhrase.containsKey(f)) {
                                // 使用finalStandardDoc以符合lambda表达式的要求
                                setField(finalStandardDoc, f, finalJsonPhrase.get(f).toString());
                            }
                        } catch (Exception e) {
                            log.error("设置字段{}时出错: {}", f, e.getMessage());
                        }
                    });

                    // 生成XML结构化内容
                    try {
                        String xmlContent = standardPdfAnalyzeService.generateXmlContent(finalJsonPhrase);
                        finalStandardDoc.setXmlContent(xmlContent);
                        log.info("标准文档XML内容生成成功");
                    } catch (Exception e) {
                        log.error("生成XML内容时出错: {}", e.getMessage());
                    }

                    // 注意：此时standardDoc已经被修改，准备好保存
                    // 由于finalStandardDoc和standardDoc指向同一个对象，所以对finalStandardDoc的修改会反映到standardDoc上
                } catch (Exception e) {
                    log.error("处理JSON解析结果时出错: {}", e.getMessage());
                }
            }

            if (Objects.isNull(jsonPhrase) || (Objects.nonNull(jsonPhrase) && jsonPhrase.size() >= 0 && !jsonPhrase.containsKey("standardNo"))) {


                JSONObject jsonObject = JSONUtil.parseObj(sysParaset.getFileTitleFormat());
                String separate = jsonObject.getStr("separate");
                String[] fields = jsonObject.getStr("fields").split(",");

                String[] title = FileUtil.mainName(zipFile.getName()).split(separate);
//            standardDocInfo.setStatus(0);   //0 未发布 1 现行有效 -1 已经废止  未发布/现行有效/已经废止

                if (title.length == fields.length) {
                    for (int i = 0; i < fields.length; i++) {
                        setField(standardDoc, fields[i], title[i]);
                    }
                }
            }
            if (StrUtil.isEmptyIfStr(standardDoc.getStandardNo())
                    || StrUtil.isEmptyIfStr(standardDoc.getStandardName())
                    || StrUtil.isEmptyIfStr(standardDoc.getStandardLevelName()))
                standardDoc.setUploadStatus("解析失败");
            else
                standardDoc.setUploadStatus("解析成功");

            if (ObjectUtil.isEmpty(uploadZipInfo.getId()) && Objects.nonNull(standardDoc.getStandardNo())) {
                StandardDoc standardDoc1 = jpaQueryFactory.select(qStandardDoc).from(qStandardDoc).where(qStandardDoc.standardNo.eq(standardDoc.getStandardNo())
                        .and(qStandardDoc.deptId.eq(sysUser.getDeptId()))).fetchFirst();
                if (Objects.nonNull(standardDoc1))
                    return R.warning("该标准已经上传，请核对！");
            }

            if (Objects.nonNull(uploadZipInfo.getId()) && Objects.nonNull(standardDoc.getStandardNo())) {
                StandardDoc standardDoc1 = jpaQueryFactory.select(qStandardDoc).from(qStandardDoc)
                        .where(qStandardDoc.standardNo.eq(standardDoc.getStandardNo()).and(qStandardDoc.id.ne(uploadZipInfo.getId()))
                                .and(qStandardDoc.deptId.eq(sysUser.getDeptId()))).fetchFirst();
                if (Objects.nonNull(standardDoc1))
                    return R.warning("该标准已经上传，请核对！");
            }

            // 保存标准文档
            standardDoc = standardDocRepository.save(standardDoc);

            // 保存规范性引用文件
            saveNormativeReferences(standardDoc, jsonPhrase);

            // 保存标准文档目录结构
            saveStandardDocToc(standardDoc, jsonPhrase);

            return R.ok(standardDoc);
        } catch (IOException e) {
            log.error("文件保存到磁盘异常: {}", zipFile.getAbsolutePath(), e);
            return R.warning("文件保存到磁盘异常");
        }
    }


    @Override
    public R createOrUpdate(StandardDocInfo standardDocInfo) {

        SysOperator sysOperator = SysUserUtils.currentUser();
        if (sysOperator != null) {
            standardDocInfo.setCreaterId(sysOperator.getId());
            standardDocInfo.setCreater(sysOperator.getOperatorName());
            standardDocInfo.setDeptId(sysOperator.getDeptId());
            standardDocInfo.setDeptName(sysOperator.getDeptName());
        }

        QStandardSystem qStandardSystem = QStandardSystem.standardSystem;
        CommonFunc.clearEntityManager(entityManager);

        StandardSystem standardSystem = jpaQueryFactory.selectFrom(qStandardSystem).where(qStandardSystem.id.eq(standardDocInfo.getSystemId())).fetchOne();

        QStandardDepartment qStandardDepartment = QStandardDepartment.standardDepartment;
        StandardDepartment standardDepartment = Objects.nonNull(standardDocInfo.getStandardDeparmentId()) ?
                jpaQueryFactory.selectFrom(qStandardDepartment).where(qStandardDepartment.id.eq(standardDocInfo.getStandardDeparmentId())).fetchOne() : null;


        standardDocInfo.setSystemName(Objects.nonNull(standardSystem) ? standardSystem.getSystemName() : null);
        standardDocInfo.setStandardDeparmentName(Objects.nonNull(standardDepartment) ? standardDepartment.getStandardDepartmentName() : null);


        QStandardDoc qStandardDoc = QStandardDoc.standardDoc;
        CommonFunc.clearEntityManager(entityManager);

        if (Objects.isNull(standardDocInfo.getId())) {
            standardDocInfo.setStatus(0);   //0 未发布 1 现行有效 -1 已经废止  未发布/现行有效/已经废止
            standardDocInfo.setStatusLabel("未发布");

            StandardDoc standardDoc = StandardDoc.builder().build();
            BeanUtils.copyProperties(standardDocInfo, standardDoc);
            standardDoc.setDelFlag(false);

            StandardDoc standardDoc1 = jpaQueryFactory.select(qStandardDoc).from(qStandardDoc)
                    .where(qStandardDoc.standardNo.eq(standardDoc.getStandardNo()).and(qStandardDoc.deptId.eq(sysOperator.getDeptId()))).fetchFirst();
            if (Objects.nonNull(standardDoc1))
                return R.warning("该标准已经上传，请核对！");

            standardDoc = standardDocRepository.save(standardDoc);

            return R.ok(standardDoc);
        } else {

            StandardDoc standardDoc = jpaQueryFactory.select(qStandardDoc).from(qStandardDoc).where(qStandardDoc.id.eq(standardDocInfo.getId())).fetchOne();
            BeanUtils.copyProperties(standardDocInfo, standardDoc, "status", "statusLabel", "createrId", "creater", "deptId", "deptName");

            StandardDoc standardDoc1 = jpaQueryFactory.select(qStandardDoc).from(qStandardDoc)
                    .where(qStandardDoc.standardNo.eq(standardDoc.getStandardNo()).and(qStandardDoc.id.ne(standardDocInfo.getId()))
                            .and(qStandardDoc.deptId.eq(sysOperator.getDeptId()))).fetchFirst();
            if (Objects.nonNull(standardDoc1))
                return R.warning("该标准已经上传，请核对！");


            standardDoc = standardDocRepository.save(standardDoc);

            return R.ok(standardDoc);
        }
    }

    @Override
    public StandardDocInfo getStandardDocDetail(Integer id) {
        QStandardDoc qStandardDoc = QStandardDoc.standardDoc;
        CommonFunc.clearEntityManager(entityManager);
        StandardDoc standardDoc = jpaQueryFactory.select(qStandardDoc).from(qStandardDoc).where(qStandardDoc.id.eq(id)).fetchOne();
        StandardDocInfo standardDocInfo = new StandardDocInfo();
        BeanUtils.copyProperties(standardDoc, standardDocInfo);

        return standardDocInfo;
    }


    @Override
    public R setStandardDocStatus(Integer id, Integer status) {
        QStandardDoc qStandardDoc = QStandardDoc.standardDoc;
        CommonFunc.clearEntityManager(entityManager);
        try {
            StandardDoc standardDoc = jpaQueryFactory.select(qStandardDoc).from(qStandardDoc).where(qStandardDoc.id.eq(id)).fetchOne();
            if (standardDoc == null)
                return R.warning("找不到该标准");
            if (status.equals(1)) {
                if (StrUtil.isEmptyIfStr(standardDoc.getStandardNo())
                        || StrUtil.isEmptyIfStr(standardDoc.getStandardName())
                        || StrUtil.isEmptyIfStr(standardDoc.getStandardLevelName()))
                    return R.warning("请先完善标准信息");

                standardDoc.setStatus(1);
                standardDoc.setStatusLabel("现行有效");
            }
            if (status.equals(-1)) {
                standardDoc.setStatus(-1);
                standardDoc.setStatusLabel("已经废止");
            }
            standardDocRepository.save(standardDoc);
            return R.ok("操作成功");
        } catch (Exception e) {
            return R.warning("操作失败:" + e.getMessage());
        }
    }

    @Override
    public R deleteDoc(Integer id) {
        QStandardDoc qStandardDoc = QStandardDoc.standardDoc;
        CommonFunc.clearEntityManager(entityManager);


        StandardDoc standardDoc = jpaQueryFactory.select(qStandardDoc).from(qStandardDoc).where(qStandardDoc.id.eq(id)).fetchOne();
        if (standardDoc == null)
            return R.warning("找不到该标准");

        if (standardDoc.getStatus() != 0)
            return R.warning("该标准已经生效，不能删除！");

        if (standardDoc.getDelFlag() != null && standardDoc.getDelFlag())
            return R.warning("标准已经删除！");


        standardDoc.setDelFlag(true);
        standardDocRepository.save(standardDoc);

        // 删除关联的规范性引用文件
        standardNormativeReferenceRepository.deleteByStandardDocId(id);

        return R.ok("删除成功");
    }

    @Override
    public StandardDocDetailResponse getStandardDocWithTocAndContent(Integer id) {
        // 1. 获取标准文档
        QStandardDoc qStandardDoc = QStandardDoc.standardDoc;
        CommonFunc.clearEntityManager(entityManager);
        StandardDoc standardDoc = jpaQueryFactory.select(qStandardDoc).from(qStandardDoc).where(qStandardDoc.id.eq(id).and(qStandardDoc.delFlag.ne(true))).fetchOne();

        if (standardDoc == null) {
            log.error("找不到ID为{}的标准文档", id);
            return null;
        }

        // 2. 获取标准文档目录
        List<StandardDocToc> tocList = standardDocTocService.getTableOfContentsByStandardDocId(id);

        // 3. 获取规范性引用文件
        List<StandardNormativeReference> normativeReferences = standardNormativeReferenceRepository.findByStandardDocId(id);

        // 4. 读取MD文件内容
        String mdContent = "";
        try {
            // 构建MD文件路径: standardDocPath + deptId + md_file_path
            String deptFolder = Objects.isNull(standardDoc.getDeptId()) ? "all" : standardDoc.getDeptId().toString();
            String mdFilePath = standardDoc.getMdFilePath();

            if (StrUtil.isNotEmpty(mdFilePath)) {
                File mdFile = new File(standardDocPath + File.separator + deptFolder + File.separator + mdFilePath);
                if (mdFile.exists()) {
                    mdContent = FileUtil.readUtf8String(mdFile);
                    log.info("成功读取标准文档MD文件: {}", mdFile.getAbsolutePath());
                } else {
                    log.warn("标准文档MD文件不存在: {}", mdFile.getAbsolutePath());
                }
            } else {
                log.warn("标准文档MD文件路径为空, 文档ID: {}", id);
            }
        } catch (Exception e) {
            log.error("读取标准文档MD文件异常: {}", e.getMessage(), e);
        }

        // 5. 构建并返回响应对象
        return StandardDocDetailResponse.builder()
                .standardDoc(standardDoc)
                .tocList(tocList)
                .mdContent(mdContent)
                .normativeReferences(normativeReferences)
                .xmlContent(standardDoc.getXmlContent())
                .build();
    }

    /**
     * 保存标准规范性引用文件
     * @param standardDoc 标准文档
     * @param jsonPhrase 解析结果
     */
    private void saveNormativeReferences(StandardDoc standardDoc, JSONObject jsonPhrase) {
        if (standardDoc == null || standardDoc.getId() == null) {
            return;
        }

        // 先删除已有的规范性引用文件
        standardNormativeReferenceRepository.deleteByStandardDocId(standardDoc.getId());

        // 如果没有规范性引用文件数据，则直接返回
        if (jsonPhrase == null || !jsonPhrase.containsKey("normativeReferences")) {
            return;
        }

        List<StandardNormativeReference> referenceList = new ArrayList<>();

        try {
            Object normativeReferencesObj = jsonPhrase.get("normativeReferences");

            // 只处理JSONArray类型
            if (normativeReferencesObj instanceof cn.hutool.json.JSONArray) {
                cn.hutool.json.JSONArray normativeReferences = (cn.hutool.json.JSONArray) normativeReferencesObj;

                for (int i = 0; i < normativeReferences.size(); i++) {
                    // 尝试直接解析JSON对象（包含standardNo和standardName字段）
                    if (normativeReferences.get(i) instanceof JSONObject) {
                        JSONObject referenceObj = normativeReferences.getJSONObject(i);
                        String standardNo = referenceObj.getStr("standardNo", "");
                        String standardName = referenceObj.getStr("standardName", "");
                        String originalText = standardNo + " " + standardName;

                        StandardNormativeReference normativeReference = StandardNormativeReference.builder()
                                .standardDocId(standardDoc.getId())
                                .standardNo(standardNo)
                                .standardName(standardName)
                                .originalText(originalText)
                                .createDate(new Date())
                                .creater(standardDoc.getCreater())
                                .createrId(standardDoc.getCreaterId())
                                .delFlag(false)
                                .build();

                        referenceList.add(normativeReference);
                    } else {
                        // 兼容原有的字符串格式处理
                        String reference = normativeReferences.getStr(i);
                        if (StrUtil.isBlank(reference)) {
                            continue;
                        }

                        // 解析标准号和标准名称
                        String standardNo = "";
                        String standardName = "";

                        // 使用正则表达式匹配多种标准号模式
                        // 1. 标准GB/T格式: GB/T 1.1-2020, GB 2635, GB/T 23219-2008
                        // 2. 企业标准格式: Q/HNYC 050, Q/HNYC 068
                        // 3. 文号格式: 国烟办综［2016]401号

                        // 尝试匹配标准格式 (GB/T 1.1-2020, GB 2635, Q/HNYC 068 等)
                        Pattern standardPattern = Pattern.compile("^([A-Z0-9]+/[A-Z]?\\s+\\d+(?:\\.\\d+)?(?:-\\d+)?)\\s+(.+)$");
                        Matcher standardMatcher = standardPattern.matcher(reference);

                        // 尝试匹配文号格式 (国烟办综［2016]401号 等)
                        Pattern docPattern = Pattern.compile("^(国烟办综［\\d+\\]\\d+号)\\s+(.+)$");
                        Matcher docMatcher = docPattern.matcher(reference);

                        if (standardMatcher.find()) {
                            // 匹配标准格式
                            standardNo = standardMatcher.group(1).trim();
                            standardName = standardMatcher.group(2).trim();
                        } else if (docMatcher.find()) {
                            // 匹配文号格式
                            standardNo = docMatcher.group(1).trim();
                            standardName = docMatcher.group(2).trim();
                        } else {
                            // 回退到简单的空格分割
                            String[] parts = reference.split("\\s+");
                            if (parts.length > 0) {
                                standardNo = parts[0].trim();
                                if (parts.length > 1) {
                                    standardNo += " " + parts[1].trim();
                                    if (parts.length > 2) {
                                        standardName = parts[2].trim();
                                    }
                                }
                            }
                        }

                        StandardNormativeReference normativeReference = StandardNormativeReference.builder()
                                .standardDocId(standardDoc.getId())
                                .standardNo(standardNo)
                                .standardName(standardName)
                                .originalText(reference)
                                .createDate(new Date())
                                .creater(standardDoc.getCreater())
                                .createrId(standardDoc.getCreaterId())
                                .delFlag(false)
                                .build();

                        referenceList.add(normativeReference);
                    }
                }
            }

            if (!referenceList.isEmpty()) {
                standardNormativeReferenceRepository.saveAll(referenceList);
                log.info("成功保存规范性引用文件，共{}条记录", referenceList.size());
            }
        } catch (Exception e) {
            log.error("保存规范性引用文件失败", e);
        }
    }

    /**
     * 保存标准文档目录结构
     * @param standardDoc 标准文档
     * @param jsonPhrase 解析结果
     */
    private void saveStandardDocToc(StandardDoc standardDoc, JSONObject jsonPhrase) {
        if (standardDoc == null || standardDoc.getId() == null) {
            return;
        }

        // 先删除已有的目录结构
        standardDocTocRepository.deleteByStandardDocId(standardDoc.getId());

        // 如果没有目录结构数据，则直接返回
        if (jsonPhrase == null || !jsonPhrase.containsKey("standardDocTocList")) {
            return;
        }

        List<StandardDocToc> tocList = new ArrayList<>();

        try {
            Object tocListObj = jsonPhrase.get("standardDocTocList");

            if (tocListObj instanceof cn.hutool.json.JSONArray) {
                cn.hutool.json.JSONArray tocArray = (cn.hutool.json.JSONArray) tocListObj;

                for (int i = 0; i < tocArray.size(); i++) {
                    JSONObject tocEntry = tocArray.getJSONObject(i);

                    StandardDocToc toc = StandardDocToc.builder()
                            .standardDocId(standardDoc.getId())
                            .sectionTitle(tocEntry.getStr("title", ""))
                            .sectionNumber(tocEntry.getStr("number", ""))
                            .level(tocEntry.getInt("level", 0))
                            .lineNumber(tocEntry.getInt("lineNumber", 0))
                            .type(tocEntry.getStr("type", ""))
                            .originalLine(tocEntry.getStr("originalLine", ""))
                            .displayOrder(i + 1) // 使用数组索引作为显示顺序
                            .createdAt(ZonedDateTime.now())
                            .updatedAt(ZonedDateTime.now())
                            .build();

                    tocList.add(toc);
                }

                if (!tocList.isEmpty()) {
                    standardDocTocRepository.saveAll(tocList);
                    log.info("成功保存标准文档目录结构，共{}条记录", tocList.size());
                }
            }
        } catch (Exception e) {
            log.error("保存标准文档目录结构失败", e);
        }
    }

    @Override
    public StandardDoc updateStandardDoc(StandardDoc standardDoc) {
        if (standardDoc == null || standardDoc.getId() == null) {
            log.error("更新标准文档失败：标准文档为空或ID为空");
            return null;
        }

        try {
            return standardDocRepository.save(standardDoc);
        } catch (Exception e) {
            log.error("更新标准文档失败: {}", e.getMessage(), e);
            return null;
        }
    }

    @Override
    public R generateAndSaveXmlContent(Integer id) {
        if (id == null) {
            return R.warning("标准文档ID不能为空");
        }

        try {
            StandardDoc standardDoc = standardDocRepository.findById(id).orElse(null);
            if (standardDoc == null) {
                return R.warning("未找到标准文档，ID: " + id);
            }

            // 构造JSON对象，包含所有需要的字段
            JSONObject jsonPhrase = buildJsonPhraseFromStandardDoc(standardDoc, id);

            // 生成XML内容
            String xmlContent = standardPdfAnalyzeService.generateXmlContent(jsonPhrase);

            // 更新数据库
            standardDoc.setXmlContent(xmlContent);
            standardDocRepository.save(standardDoc);

            log.info("成功为标准文档生成XML内容，ID: {}, 标准号: {}", id, standardDoc.getStandardNo());
            return R.ok("XML内容生成成功");

        } catch (Exception e) {
            log.error("为标准文档生成XML内容失败，ID: {}, 错误: {}", id, e.getMessage(), e);
            return R.warning("生成XML内容失败: " + e.getMessage());
        }
    }

    @Override
    public R batchGenerateAndSaveXmlContent(List<Integer> ids) {
        int successCount = 0;
        int failCount = 0;
        List<String> failMessages = new ArrayList<>();

        try {
            List<StandardDoc> docsToProcess;

            if (ids != null && !ids.isEmpty()) {
                // 处理指定的ID列表
                docsToProcess = standardDocRepository.findAllById(ids);
                log.info("开始批量生成XML内容，指定处理 {} 个标准文档", ids.size());
            } else {
                // 处理所有没有XML内容的文档
                QStandardDoc qStandardDoc = QStandardDoc.standardDoc;
                docsToProcess = jpaQueryFactory
                        .select(qStandardDoc)
                        .from(qStandardDoc)
                        .where(qStandardDoc.xmlContent.isNull()
                                .or(qStandardDoc.xmlContent.eq(""))
                                .and(qStandardDoc.delFlag.ne(true)))
                        .fetch();
                log.info("开始批量生成XML内容，共发现 {} 个没有XML内容的标准文档", docsToProcess.size());
            }

            // 逐个处理
            for (StandardDoc standardDoc : docsToProcess) {
                try {
                    // 构造JSON对象
                    JSONObject jsonPhrase = buildJsonPhraseFromStandardDoc(standardDoc, standardDoc.getId());

                    // 生成XML内容
                    String xmlContent = standardPdfAnalyzeService.generateXmlContent(jsonPhrase);

                    // 更新数据库
                    standardDoc.setXmlContent(xmlContent);
                    standardDocRepository.save(standardDoc);

                    successCount++;
                    log.debug("成功生成XML，ID: {}, 标准号: {}", standardDoc.getId(), standardDoc.getStandardNo());

                } catch (Exception e) {
                    failCount++;
                    String errorMsg = String.format("ID: %d, 标准号: %s, 错误: %s",
                            standardDoc.getId(), standardDoc.getStandardNo(), e.getMessage());
                    failMessages.add(errorMsg);
                    log.error("生成XML失败，{}", errorMsg);
                }
            }

            // 构建返回结果
            JSONObject result = new JSONObject();
            result.set("successCount", successCount);
            result.set("failCount", failCount);
            if (!failMessages.isEmpty()) {
                result.set("failDetails", failMessages);
            }

            log.info("批量生成XML内容完成，成功: {}, 失败: {}", successCount, failCount);
            return R.ok("批量生成完成");

        } catch (Exception e) {
            log.error("批量生成XML内容异常: {}", e.getMessage(), e);
            return R.warning("批量生成异常: " + e.getMessage());
        }
    }

    /**
     * 从StandardDoc实体构造用于XML生成的JSON对象
     * @param standardDoc 标准文档实体
     * @param id 标准文档ID
     * @return 包含所有必要字段的JSONObject
     */
    private JSONObject buildJsonPhraseFromStandardDoc(StandardDoc standardDoc, Integer id) {
        JSONObject jsonPhrase = new JSONObject();

        // 基本信息
        jsonPhrase.set("standardNo", standardDoc.getStandardNo());
        jsonPhrase.set("standardName", standardDoc.getStandardName());
        jsonPhrase.set("standardEnName", standardDoc.getStandardEnName());
        jsonPhrase.set("standardLevelName", standardDoc.getStandardLevelName());
        jsonPhrase.set("standardStatus", standardDoc.getStandardStatus());
        jsonPhrase.set("publishDate", standardDoc.getPublishDate());
        jsonPhrase.set("implementDate", standardDoc.getImplementDate());
        jsonPhrase.set("publishUnit", standardDoc.getPublishUnit());
        jsonPhrase.set("proposeUnit", standardDoc.getProposeUnit());
        jsonPhrase.set("draftUnit", standardDoc.getDraftUnit());
        jsonPhrase.set("drafter", standardDoc.getDrafter());
        jsonPhrase.set("scope", standardDoc.getScope());
        jsonPhrase.set("introduction", standardDoc.getIntroduction());
        jsonPhrase.set("preface", standardDoc.getPreface());
        jsonPhrase.set("referenceDocs", standardDoc.getReferenceDocs());
        jsonPhrase.set("normativeReferences", standardDoc.getNormativeReferences());
        jsonPhrase.set("termsAndDefinitions", standardDoc.getTermsAndDefinitions());

        // 目录结构
        try {
            List<StandardDocToc> tocList = standardDocTocRepository.findByStandardDocIdOrderByDisplayOrder(id);
            if (tocList != null && !tocList.isEmpty()) {
                cn.hutool.json.JSONArray tocArray = new cn.hutool.json.JSONArray();
                for (StandardDocToc toc : tocList) {
                    JSONObject tocEntry = new JSONObject();
                    tocEntry.set("title", toc.getSectionTitle());
                    tocEntry.set("number", toc.getSectionNumber());
                    tocEntry.set("level", toc.getLevel());
                    tocEntry.set("lineNumber", toc.getLineNumber());
                    tocEntry.set("type", toc.getType());
                    tocEntry.set("originalLine", toc.getOriginalLine());
                    tocArray.add(tocEntry);
                }
                jsonPhrase.set("standardDocTocList", tocArray);
            }
        } catch (Exception e) {
            log.warn("获取标准目录失败，ID: {}, 错误: {}", id, e.getMessage());
        }

        return jsonPhrase;
    }
}