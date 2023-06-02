<script lang="ts">
import Panel from './Panel.vue';

export default {
    components: {
        Panel
    }
}
</script>

<template>
    <Panel class="Privacy" label="Privacy">
        <p>
            This Privacy Statement explains how we collect, use, disclose,
            and safeguard your personal information when you use our services.
            We are committed to protecting your privacy and ensuring the
            security of your personal information.
        </p>

        <h2>1. Information We Collect</h2>

        <p>
            We may collect personal information from you when you create a
            personal account. The types of personal information we may collect
            include:
        </p>

        <ul>
            <li>Contact information (such as name, email address)</li>
            <li>Login credentials (such as username, password)</li>
        </ul>

        <h2>2. How We Use Your Information</h2>

        <p>
            We use the collected information for the following purposes:
        </p>

        <ul>
            <li>Provide and maintain our services</li>
            <li>Process and fulfill your requests</li>
            <li>Personalize your experience with our services</li>
            <li>Improve our services and develop new features</li>
            <li>Communicate with you, including customer support</li>
            <li>Ensure the security and integrity of our services</li>
        </ul>

        <h2>3. Information Sharing and Disclosure</h2>

        <p>
            We may share your personal information in the following
            circumstances:
        </p>

        <ul>
            <li>With your consent or at your direction</li>
            <li>
                With trusted third-party service providers who assist us in
                operating our services and conducting our business
            </li>
            <li>
                In response to a legal request, such as a court order or
                government investigation
            </li>
            <li>
                To protect our rights, property, or safety, as well as the
                rights, property, or safety of others
            </li>
        </ul>

        <h2>4. Data Security</h2>

        <p>
            We implement appropriate security measures to protect your personal
            information from unauthorized access, use, disclosure, or alteration.
            However, please note that no method of transmission over the internet
            or electronic storage is 100% secure, and we cannot guarantee absolute
            security.
        </p>

        <h2>5. Your Rights and Choices</h2>

        <p>
            You have certain rights and choices regarding the personal information
            we collect and process. These may include:
        </p>

        <ul>
            <li>
                Access and rectification: You can request access to your personal
                information and ask us to correct any inaccuracies.
            </li>
            <li>
                Data deletion: You can request the deletion of your personal
                information, subject to certain legal limitations and requirements.
            </li>
        </ul>

        <h2>6. Children's Privacy</h2>

        <p>
            Our services are not directed to individuals under the age of 13
            (or a higher age if required by applicable law). We do not knowingly
            collect or solicit personal information from children. If we become
            aware that we have collected personal information from a child without
            parental consent, we will take steps to remove that information from
            our records.
        </p>

        <h2>7. Updates to this Privacy Statement</h2>

        <p>
            We may update this Privacy Statement from time to time. We will notify
            you of any material changes by posting the updated Privacy Statement on
            our website or through other communication channels.
        </p>

        <h2>8. Contact Us</h2>

        <p>
            If you have any questions, concerns, or requests regarding this Privacy
            Statement or our privacy practices, please contact us using the following
            information:
        </p>

        <p>StudyHub BXL</p>
        <p>studyhubxl@contact.com</p>
    </Panel>
</template>


<style scoped>
.Privacy {
    opacity: 2;
    width: 33%;
}

.Privacy.expanded {
    opacity: 2;
    width: 450px;
}
</style>
