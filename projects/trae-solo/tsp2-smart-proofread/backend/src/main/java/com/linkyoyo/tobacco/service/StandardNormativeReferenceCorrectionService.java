package com.linkyoyo.tobacco.service;

import com.linkyoyo.tobacco.entity.QStandardDoc;
import com.linkyoyo.tobacco.entity.StandardDoc;
import com.linkyoyo.tobacco.entity.StandardNormativeReference;
import com.linkyoyo.tobacco.repository.StandardDocRepository;
import com.linkyoyo.tobacco.repository.StandardNormativeReferenceRepository;
import com.querydsl.jpa.impl.JPAQueryFactory;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import javax.persistence.EntityManager;
import java.util.List;

/**
 * 标准规范性引用修正服务
 * 负责修正规范性引用文件中的标准号
 */
@Service
public class StandardNormativeReferenceCorrectionService {
    
    private static final Logger log = LoggerFactory.getLogger(StandardNormativeReferenceCorrectionService.class);
    
    @Autowired
    private StandardDocRepository standardDocRepository;

    @Autowired
    private StandardNormativeReferenceRepository standardNormativeReferenceRepository;

    @Autowired
    private JPAQueryFactory jpaQueryFactory;

    @Autowired
    private EntityManager entityManager;
    
    /**
     * 修正规范引用标准号
     * 查询 is_corrected_normative_reference = false 的 StandardDoc，
     * 对其规范性引用文件进行标准号修正
     */
    @Transactional
    public void correctNormativeReferences() {
        log.info("开始修正规范引用标准号...");
        
        try {
            // 查询未修正规范引用的标准文档
            QStandardDoc qStandardDoc = QStandardDoc.standardDoc;
            List<StandardDoc> uncorrectedDocs = jpaQueryFactory.select(qStandardDoc)
                    .from(qStandardDoc)
                    .where(qStandardDoc.isCorrectedNormativeReference.eq(false)
                            .and(qStandardDoc.delFlag.ne(true)))
                    .fetch();

            log.info("找到 {} 个需要修正规范引用的标准文档", uncorrectedDocs.size());
            
            int correctedCount = 0;
            for (StandardDoc standardDoc : uncorrectedDocs) {
                try {
                    // 查询该标准文档的规范性引用文件
                    List<StandardNormativeReference> normativeReferences = 
                            standardNormativeReferenceRepository.findByStandardDocId(standardDoc.getId());
                    
                    boolean hasCorrections = false;
                    
                    // 循环处理每个规范性引用
                    for (StandardNormativeReference reference : normativeReferences) {
                        if (reference.getStandardNo() != null && !reference.getStandardNo().trim().isEmpty()) {
                            // 查询匹配的标准文档
                            List<StandardDoc> matchingDocs = jpaQueryFactory.select(qStandardDoc)
                                    .from(qStandardDoc)
                                    .where(qStandardDoc.deptId.eq(standardDoc.getDeptId())
                                            .and(qStandardDoc.status.eq(1))
                                            .and(qStandardDoc.standardNo.like(reference.getStandardNo() + "%"))
                                            .and(qStandardDoc.delFlag.ne(true)))
                                    .orderBy(qStandardDoc.standardNo.desc())
                                    .limit(1)
                                    .fetch();
                            
                            // 如果找到匹配的标准文档，取第一条的标准号进行修正
                            if (!matchingDocs.isEmpty()) {
                                StandardDoc matchingDoc = matchingDocs.get(0);
                                String originalStandardNo = reference.getStandardNo();
                                reference.setStandardNo(matchingDoc.getStandardNo());
                                standardNormativeReferenceRepository.save(reference);
                                hasCorrections = true;
                                
                                log.debug("修正规范引用: {} -> {}", originalStandardNo, matchingDoc.getStandardNo());
                            }
                        }
                    }
                    
                    // 标记该标准文档的规范引用已修正
                    standardDoc.setIsCorrectedNormativeReference(true);
                    standardDocRepository.save(standardDoc);
                    
                    if (hasCorrections) {
                        correctedCount++;
                    }
                    
                } catch (Exception e) {
                    log.error("修正标准文档规范引用失败，ID: {}, 错误: {}", standardDoc.getId(), e.getMessage(), e);
                }
            }
            
            log.info("规范引用修正完成，成功修正 {} 个标准文档", correctedCount);
            
        } catch (Exception e) {
            log.error("修正规范引用任务执行失败: {}", e.getMessage(), e);
        }
    }
}
