package com.linkyoyo.tobacco.controller;


import cn.hutool.core.io.FileUtil;
import cn.hutool.core.util.ObjectUtil;
import cn.hutool.json.JSONObject;
import com.linkyoyo.tobacco.annotation.SysOperaLog;
import com.linkyoyo.tobacco.entity.QStandardDoc;
import com.linkyoyo.tobacco.entity.StandardDoc;
import com.linkyoyo.tobacco.entity.SysOperator;
import com.linkyoyo.tobacco.info.StandardDocInfo;
import com.linkyoyo.tobacco.info.UploadSingleFileInfo;
import com.linkyoyo.tobacco.info.UploadZipFileInfo;
import com.linkyoyo.tobacco.query.StandardDocQuery;
import com.linkyoyo.tobacco.repository.StandardDocRepository;
import com.linkyoyo.tobacco.result.R;
import com.linkyoyo.tobacco.service.StandardDocService;
import com.linkyoyo.tobacco.service.StandardNormativeReferenceCorrectionService;

import com.linkyoyo.tobacco.support.CommonFunc;
import com.linkyoyo.tobacco.util.ResponseEntityUtil;
import com.linkyoyo.tobacco.util.SysUserUtils;
import com.querydsl.jpa.impl.JPAQueryFactory;

import javax.persistence.EntityManager;
import javax.persistence.PersistenceContext;
import lombok.extern.slf4j.Slf4j;
import org.apache.pdfbox.Loader;
import org.apache.pdfbox.pdmodel.PDDocument;
import org.apache.pdfbox.rendering.PDFRenderer;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.core.io.InputStreamResource;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.context.request.RequestContextHolder;
import org.springframework.web.context.request.ServletRequestAttributes;
import org.springframework.web.servlet.mvc.method.annotation.SseEmitter;

import javax.imageio.ImageIO;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import javax.validation.constraints.NotNull;
import java.awt.image.BufferedImage;
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.Base64;
import java.util.List;
import java.util.Map;
import java.util.Objects;


@RequestMapping("/standardDoc")
@RestController
@Slf4j
@Validated
public class StandardDocController {
    @Value("${paraSet.uploadPath}")
    private String uploadPath ;

    @Value("${paraSet.unzipPath}")
    private String unzipPath ;


    @Autowired
    private StandardDocService standardDocService ;

    @Autowired
    private StandardDocRepository standardDocRepository;

    @Autowired
    private StandardNormativeReferenceCorrectionService correctionService;

    @Autowired
    private JPAQueryFactory jpaQueryFactory;

    @PersistenceContext
    private EntityManager entityManager;

    @GetMapping("/list")
    @SysOperaLog(value="获取标准文档列表", id =  "标准管理")
    public R getStandardDocList( StandardDocQuery standardDocQuery)
    {

        SysOperator sysOperator =  SysUserUtils.currentUser();
        if (Objects.nonNull(sysOperator)) {
            if (sysOperator.getGroupCode().equals("003"))
                standardDocQuery.setEnterpriseId(sysOperator.getDeptId());
        }
        return standardDocService.queryByCondition(standardDocQuery);

//        return  R.ok(standardDocService.getStandardDocList(standardDocQuery)) ;
    }
    
    /**
     * 高级查询标准文档
     * 支持以下查询条件：
     * - standardLevelName: 标准级别
     * - systemId: 标准子体系
     * - standardDeparmentId: 归口部门
     * - drafter: 起草人
     * - draftUnit: 起草单位
     * - publishDate: 发布日期（匹配年月）
     * - implementDate: 实施日期（左匹配）
     * - status: 标准状态（0/1/-1 未发布/现行有效/已经废止）
     * - standardName: 标准名称
     * - scope: 适用范围
     * 
     * @param standardDocQuery 包含condition字段的查询对象，condition为JSON字符串
     * @return 查询结果，包含分页信息
     */
    @PostMapping("/advancedSearch")
    @SysOperaLog(value="高级查询标准文档", id = "标准管理")
    public R advancedSearchStandardDoc(@RequestBody StandardDocQuery standardDocQuery) {
        SysOperator sysOperator = SysUserUtils.currentUser();
        if (Objects.nonNull(sysOperator)) {

//            if (sysOperator.getGroupCode().equals("003"))
                standardDocQuery.setEnterpriseId(sysOperator.getDeptId());
        }
        return standardDocService.advancedQuery(standardDocQuery);
    }
    



    @GetMapping("/listByEnterprise")
    @SysOperaLog(value="获取标准文档列表", id =  "标准管理")
    public R getListByEnterprise( StandardDocQuery standardDocQuery)
    {

        SysOperator sysOperator =  SysUserUtils.currentUser();
        if (Objects.nonNull(sysOperator)) {
            if (sysOperator.getGroupCode().equals("003"))
              standardDocQuery.setEnterpriseId(sysOperator.getDeptId());
        }

        return  R.ok(standardDocService.getListByEnterprise(standardDocQuery)) ;
    }

    @GetMapping("/listByEnterpriseAll")
    @SysOperaLog(value="获取标准文档列表", id =  "标准管理")
    public R getListByEnterpriseAll( StandardDocQuery standardDocQuery)
    {

        SysOperator sysOperator =  SysUserUtils.currentUser();
        if (Objects.nonNull(sysOperator)) {
            if (sysOperator.getGroupCode().equals("003"))
                standardDocQuery.setEnterpriseId(sysOperator.getDeptId());
        }

        return  R.ok(standardDocService.getListByEnterprise(standardDocQuery)) ;
    }

    @GetMapping("/listDetail")
    public R getStandardDocDetail(StandardDocInfo standardDocInfo)
    {
        return  R.ok(standardDocService.getStandardDocDetail(standardDocInfo.getId())) ;
    }

    /**
     * 获取标准文档详情，包含标准文档、目录和内容
     * @param id 标准文档ID
     * @return 标准文档详情响应
     */
    @GetMapping("/detail/{id}")
    @SysOperaLog(value="获取标准文档详情", id = "标准管理")
    public R getStandardDocWithTocAndContent(@NotNull @PathVariable Integer id) {
        return R.ok(standardDocService.getStandardDocWithTocAndContent(id));
    }

    /**
     * 根据标准号查询标准文档详情
     * @param standardNo 标准号
     * @return 标准文档详情响应
     */
    @GetMapping("/findByStandardNo")
    @SysOperaLog(value="根据标准号查询标准文档", id = "标准管理")
    public R findByStandardNo(@NotNull @RequestParam String standardNo, @RequestParam(required = false) Integer deptId) {
        QStandardDoc qStandardDoc = QStandardDoc.standardDoc;
        CommonFunc.clearEntityManager(entityManager);

        // 构建查询条件
        var whereCondition = qStandardDoc.standardNo.eq(standardNo)
                .and(qStandardDoc.delFlag.ne(true));

        // 如果传入了 deptId，则添加部门条件
        if (deptId != null) {
            whereCondition = whereCondition.and(qStandardDoc.deptId.eq(deptId));
        }

        StandardDoc standardDoc = jpaQueryFactory.select(qStandardDoc)
                .from(qStandardDoc)
                .where(whereCondition)
                .fetchFirst();

        if (standardDoc == null) {
            String message = deptId != null ?
                "找不到标准号为" + standardNo + "且部门ID为" + deptId + "的标准文档" :
                "找不到标准号为" + standardNo + "的标准文档";
            return R.warning(message);
        }

        return R.ok(standardDocService.getStandardDocWithTocAndContent(standardDoc.getId()));
    }


    @PostMapping ("/update")
    @SysOperaLog(value="更新标准文档", id =  "标准管理")
    public R updateData(@RequestBody StandardDocInfo standardDocInfo)
    {
        if (standardDocInfo.getId()!=null)
        {
            StandardDoc standardDoc = standardDocRepository.getOne(standardDocInfo.getId());
            if (standardDoc!=null)
            {
                if (standardDoc.getStatus().equals(1))
                    return  R.warning("标准已经生效，无法修改!");
                if (standardDoc.getStatus().equals(-1))
                    return  R.warning("标准已经废止，无法修改!");

                if(standardDoc.getDelFlag())
                    return  R.warning("标准已经删除，无法修改!");

            }
        }

        return standardDocService.createOrUpdate(standardDocInfo);
    }

    @PostMapping("/uploadZip")
    @SysOperaLog(value="上传标准文档压缩包", id =  "标准管理")
    public R uploadZip(@Validated UploadZipFileInfo uploadZipInfo) {

        HttpServletRequest request = ((ServletRequestAttributes) Objects
                .requireNonNull(RequestContextHolder.getRequestAttributes())).getRequest();
        Object obj = request.getSession().getAttribute("userId");
        if (!ObjectUtil.isEmpty(obj))
            uploadZipInfo.setUserId(Integer.valueOf(obj.toString()));


        if (ObjectUtil.isEmpty(uploadZipInfo.getUserId()) )   //&&  ObjectUtil.isEmpty(uploadFileInfo.getMobilePhone())
        {
            return R.warning("用户Id不能为空！");
        }

        if (ObjectUtil.isEmpty(uploadZipInfo.getSystemId()) )
        {
            return R.warning("体系id不能为空！");
        }


        if (ObjectUtil.isEmpty(uploadZipInfo.getFile()))
        {
            return R.warning("文件不能为空！");
        }

        if (!Arrays.asList("zip", "rar").contains(FileUtil.extName(uploadZipInfo.getFile().getOriginalFilename()))) {
            return R.warning("文件格式不正确！");
        }

        return standardDocService.uploadZip(uploadZipInfo);

    }

    @PostMapping("/uploadFile")
    @SysOperaLog(value = "上传标准文档pdf文件" , id =  "标准管理")
    public R uploadFile(UploadSingleFileInfo uploadFileInfo) throws IOException {

        HttpServletRequest request = ((ServletRequestAttributes) Objects
                .requireNonNull(RequestContextHolder.getRequestAttributes())).getRequest();
        Object obj = request.getSession().getAttribute("userId");
        if (!ObjectUtil.isEmpty(obj))
          uploadFileInfo.setUserId(Integer.valueOf(obj.toString()));
        if (ObjectUtil.isEmpty(uploadFileInfo.getUserId()) )   //&&  ObjectUtil.isEmpty(uploadFileInfo.getMobilePhone())
        {
            return R.warning("用户Id不能为空！");
        }

        if (ObjectUtil.isEmpty(uploadFileInfo.getSystemId()) )
        {
            return R.warning("体系id不能为空！");
        }

        if (ObjectUtil.isEmpty(uploadFileInfo.getFile()))
        {
            return R.warning("文件不能为空！");
        }

        if (!Arrays.asList("pdf", "docx","doc","xml").contains(FileUtil.extName(uploadFileInfo.getFile().getOriginalFilename()))) {
            return R.warning("文件格式不正确！");
        }

        return standardDocService.uploadFile(uploadFileInfo);

    }


    @PostMapping("/phraseImage")
    public SseEmitter phraseImage(UploadSingleFileInfo uploadFileInfo) throws IOException {
        SseEmitter sseEmitter = new SseEmitter();

        if (ObjectUtil.isEmpty(uploadFileInfo.getFile()))
        {
            return null;
        }

        String extName = FileUtil.extName(uploadFileInfo.getFile().getOriginalFilename()) ;

        if (!Arrays.asList("pdf", "png","jpeg","jpg").contains(FileUtil.extName(uploadFileInfo.getFile().getOriginalFilename()))) {
            return null;
        }

        String base64 = "";

        if(extName.equals("pdf"))
        {
            PDDocument pdDocument = Loader.loadPDF(uploadFileInfo.getFile().getBytes());
            PDFRenderer renderer = new PDFRenderer(pdDocument);
            BufferedImage image = renderer.renderImageWithDPI(0, 70); // 设置图像分辨率为70 DPI

            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            ImageIO.write(image, "png", baos); // 可以改为需要的图片格式，如"jpg"
            byte[] imageBytes = baos.toByteArray();
            base64 =   Base64.getEncoder().encodeToString(imageBytes) ;

        }
        else
        {
            BufferedImage image = ImageIO.read(uploadFileInfo.getFile().getInputStream());
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            ImageIO.write(image, extName, baos); // 可以改为需要的图片格式，如 "jpg"
            byte[] imageBytes = baos.toByteArray();
            base64 = Base64.getEncoder().encodeToString(imageBytes);
        }




        CommonFunc.CallOpenAiBySse(base64,sseEmitter);

        return sseEmitter;

    }


    @GetMapping("/getFile/{id}")
    @SysOperaLog(value="获取标准文档pdf文件", id =  "标准管理")
    public void getPicture(@NotNull  @PathVariable Integer id, HttpServletResponse response) throws IOException {
        response.setContentType("application/pdf");
        File file = standardDocService.getFile(id);
        if (file != null)
            FileUtil.writeToStream(file,response.getOutputStream());

//        return ResponseEntityUtil.resource(file);
        else {
            response.setContentType("text/html;charset=utf-8");
            response.getOutputStream().write("文件不存在".getBytes());
//            return null;
        }

    }

    @GetMapping("/downFile/{id}")
    @SysOperaLog(value="下载标准文档pdf文件", id =  "标准管理")
    public ResponseEntity<InputStreamResource> downFile(@NotNull  @PathVariable Integer id, HttpServletResponse response) throws IOException {
//        response.setContentType("application/pdf");
        File file = standardDocService.getFile(id);
        if (file != null)

            return ResponseEntityUtil.resource(file);
        else {

           return ResponseEntityUtil.noFile();
            // return ResponseEntity.badRequest().build();

        }
    }

    @RequestMapping("/setStatus")
    @SysOperaLog(value="设置标准文档状态", id =  "标准管理")
    public R setStatus( @NotNull Integer id, @NotNull  Integer status) {
        if (status != -1 && status != 1)
            return R.warning("状态值不正确！");

        return standardDocService.setStandardDocStatus(id, status);
    }

    @RequestMapping("/delete")
    @SysOperaLog(value="删除标准文档", id =  "标准管理")
    public R deleteDoc( @NotNull Integer id) {

        return standardDocService.deleteDoc(id);
    }


    @GetMapping("/downTotal/{id}")
    @SysOperaLog(value="下载标准文档统计文件", id =  "标准管理")
    public ResponseEntity<InputStreamResource> downTotalFile(@NotNull  @PathVariable Integer id, HttpServletResponse response) throws IOException {
//        response.setContentType("application/pdf");
        File file = standardDocService.getSumFile(id);
        if (file != null)

            return ResponseEntityUtil.resource(file);
        else {

//            return ResponseEntity.badRequest().build();
            return ResponseEntityUtil.noFile();

        }
    }


    @GetMapping("/downDetail/{id}/{systemId}")
    @SysOperaLog(value="下载标准明细文件", id =  "标准管理")
    public ResponseEntity<InputStreamResource> downDetailFile(@NotNull  @PathVariable Integer id,@NotNull  @PathVariable Integer systemId, HttpServletResponse response) throws IOException {
//        response.setContentType("application/pdf");
        File file = standardDocService.getDetailFile(id,systemId);
        if (file != null)

            return ResponseEntityUtil.resource(file);
        else {

            return ResponseEntityUtil.noFile();

//            return ResponseEntity.badRequest().build();

        }
    }

    @PostMapping("/uploadPng")
    public R uploadAi()
    {
        CommonFunc.callAi(new File("D:\\work\\河南烟草\\pdf解析\\1015.png")) ;
        return R.ok() ;
    }


    @PostMapping("/Ai")
    public R AiPhraseImage(UploadSingleFileInfo uploadFileInfo) throws IOException {


        if (ObjectUtil.isEmpty(uploadFileInfo.getFile()))
        {
            return null;
        }

        String extName = FileUtil.extName(uploadFileInfo.getFile().getOriginalFilename()) ;

        if (!Arrays.asList("pdf", "png","jpeg","jpg").contains(FileUtil.extName(uploadFileInfo.getFile().getOriginalFilename()))) {
            return null;
        }

        String base64 = "";

        if(extName.equals("pdf"))
        {
            PDDocument pdDocument = Loader.loadPDF(uploadFileInfo.getFile().getBytes());
            PDFRenderer renderer = new PDFRenderer(pdDocument);
            BufferedImage image = renderer.renderImageWithDPI(0, 70); // 设置图像分辨率为70 DPI

            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            ImageIO.write(image, "png", baos); // 可以改为需要的图片格式，如"jpg"
            byte[] imageBytes = baos.toByteArray();
            base64 =   Base64.getEncoder().encodeToString(imageBytes) ;

        }
        else
        {
            BufferedImage image = ImageIO.read(uploadFileInfo.getFile().getInputStream());
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            ImageIO.write(image, extName, baos); // 可以改为需要的图片格式，如 "jpg"
            byte[] imageBytes = baos.toByteArray();
            base64 = Base64.getEncoder().encodeToString(imageBytes);
        }




//        CommonFunc.CallOpenAiBySse(base64,sseEmitter);

        return R.ok(CommonFunc.callNewOpenAi(base64));

    }

    /**
     * 测试修正规范引用标准号
     * 手动触发修正规范引用的逻辑
     * @return 执行结果
     */
    @PostMapping("/test/correctNormativeReferences")
    @SysOperaLog(value="测试修正规范引用标准号", id = "标准管理")
    public R testCorrectNormativeReferences() {
        try {
            log.info("手动触发修正规范引用标准号测试...");
            correctionService.correctNormativeReferences();
            return R.ok("修正规范引用标准号执行完成，请查看日志了解详细结果");
        } catch (Exception e) {
            log.error("修正规范引用标准号测试失败: {}", e.getMessage(), e);
            return R.warning("修正规范引用标准号执行失败: " + e.getMessage());
        }
    }

    /**
     * 获取标准文档的XML结构化内容
     * @param id 标准文档ID
     * @return XML内容
     */
    @GetMapping("/xml/{id}")
    @SysOperaLog(value="获取标准文档XML内容", id = "标准管理")
    public R getStandardDocXml(@NotNull @PathVariable Integer id) {
        QStandardDoc qStandardDoc = QStandardDoc.standardDoc;
        CommonFunc.clearEntityManager(entityManager);
        StandardDoc standardDoc = jpaQueryFactory.select(qStandardDoc).from(qStandardDoc)
                .where(qStandardDoc.id.eq(id).and(qStandardDoc.delFlag.ne(true))).fetchOne();
        
        if (standardDoc == null) {
            return R.warning("标准文档不存在");
        }
        return R.ok(standardDoc.getXmlContent());
    }

    /**
     * 为单个标准文档生成并保存XML结构化内容
     * 用于历史数据补全XML内容
     * 
     * @param id 标准文档ID
     * @return 执行结果，包含生成的XML内容
     */
    @PostMapping("/generateXml/{id}")
    @SysOperaLog(value="生成标准文档XML内容", id = "标准管理")
    public R generateXmlContent(@NotNull @PathVariable Integer id) {
        return standardDocService.generateAndSaveXmlContent(id);
    }

    /**
     * 批量为标准文档生成并保存XML结构化内容
     * 用于历史数据批量补全XML内容
     * 
     * @param requestBody 包含ids数组的请求体，为空时处理所有没有XML的文档
     * @return 执行结果，包含成功/失败数量
     */
    @PostMapping("/generateXml/batch")
    @SysOperaLog(value="批量生成标准文档XML内容", id = "标准管理")
    public R batchGenerateXmlContent(@RequestBody(required = false) Map<String, List<Integer>> requestBody) {
        List<Integer> ids = null;
        if (requestBody != null && requestBody.containsKey("ids")) {
            ids = requestBody.get("ids");
        }
        return standardDocService.batchGenerateAndSaveXmlContent(ids);
    }

}
